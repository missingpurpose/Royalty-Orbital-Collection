# 🎯 Alkane RoyaltyNFT: Unavoidable Royalties Deployment Guide

This guide explains how to deploy and test the Alkane RoyaltyNFT collection with mandatory 5% royalties that cannot be bypassed.

## 📋 Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Prerequisites](#prerequisites)
3. [Configuration](#configuration)
4. [Deployment Steps](#deployment-steps)
5. [Testing Guide](#testing-guide)
6. [Marketplace Integration](#marketplace-integration)
7. [Troubleshooting](#troubleshooting)

---

## 🏗️ Architecture Overview

### System Components
1. **Collection Contract** (`alkane-pandas-collection-MY FORK/`)
   - Factory that mints NFTs
   - Configures royalty settings
   - Receives primary mint payments (0.0001 BTC per NFT)
   - Receives secondary sale royalties (5%)

2. **Child Contract** (`alkane-pandas-child-main/`)
   - Individual NFT instances
   - Enforces mandatory royalty payments
   - Only allows transfers through `TransferWithRoyalty` opcode

### Payment Flow
```
Buyer → [Sale Price + 5% Royalty] → NFT Contract
NFT Contract → [Sale Price] → Seller
NFT Contract → [5% Royalty] → Collection Contract
```

### Safety Guarantees
- ✅ **Buyer Protection**: Failed transactions don't lose money
- ✅ **Seller Protection**: NFTs can't be lost in failed transfers
- ✅ **Creator Protection**: Royalties cannot be bypassed

---

## 🔧 Prerequisites

### Required Tools
```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install OYL SDK (replace with actual installation method)
npm install -g oyl-sdk
```

### Network Setup
```bash
# Start regtest environment (example)
oyl regtest start

# Verify connection
oyl provider info -p regtest
```

---

## ⚙️ Configuration

### Step 1: Configure Royalty Recipients

Edit `alkane-pandas-collection-MY FORK/src/lib.rs`:

```rust
/// Royalty configuration
const ROYALTY_PERCENTAGE: u128 = 500; // 5% in basis points
const ROYALTY_RECIPIENT: AlkaneId = AlkaneId { 
  block: 2, 
  tx: 0x[YOUR_WALLET_TX_ID] // ⚠️ UPDATE THIS
};

/// Primary sales configuration  
const PRIMARY_SALES_RECIPIENT: AlkaneId = AlkaneId { 
  block: 2, 
  tx: 0x[YOUR_WALLET_TX_ID] // ⚠️ UPDATE THIS
};
```

### Step 2: Configure Template Reference

The collection contract references the child contract template:

```rust
const ROYALTY_NFT_ORBITAL_TEMPLATE_ID: u128 = 0x378; // Will be updated after deployment
```

---

## 🚀 Deployment Steps

### Phase 1: Deploy Child Contract (NFT Template)

```bash
# Navigate to child contract
cd /path/to/alkane-pandas-child-main/

# Build WebAssembly binary
cargo build --target wasm32-unknown-unknown --release

# Verify build
ls target/wasm32-unknown-unknown/release/alkane_pandas_child.wasm

# Deploy to regtest
oyl alkane new-contract \
  -c ./target/wasm32-unknown-unknown/release/alkane_pandas_child.wasm \
  -data 1,0 \
  -p regtest

# ⚠️ IMPORTANT: Note the transaction ID from deployment output
# Example output: "Contract deployed with tx: 0xabc123..."
```

### Phase 2: Update Collection Contract

```bash
# Navigate to collection contract
cd /path/to/alkane-pandas-collection-MY\ FORK/

# Update template ID in src/lib.rs
# Replace 0x378 with the transaction ID from Phase 1:
const ROYALTY_NFT_ORBITAL_TEMPLATE_ID: u128 = 0xabc123; // Use actual TX ID
```

### Phase 3: Deploy Collection Contract

```bash
# Build WebAssembly binary
cargo build --target wasm32-unknown-unknown --release

# Verify build
ls target/wasm32-unknown-unknown/release/alkane_pandas.wasm

# Deploy to regtest
oyl alkane new-contract \
  -c ./target/wasm32-unknown-unknown/release/alkane_pandas.wasm \
  -data 1,0 \
  -p regtest

# ⚠️ IMPORTANT: Note this transaction ID for testing
# Example: "Collection deployed with tx: 0xdef456..."
```

---

## 🧪 Testing Guide

### Environment Variables
```bash
export COLLECTION_ID="0xdef456"  # From Phase 3 deployment
export CHILD_TEMPLATE_ID="0xabc123"  # From Phase 1 deployment
export NETWORK="regtest"
```

### Test 1: Mint NFT (Should Succeed)

```bash
# Mint with 0.0001 BTC payment
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'", 
    "opcode":77,
    "incoming_alkanes": [
      {"id":"0:0", "value":10000}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ Success with new NFT created
# Note the NFT ID from response for transfer tests
```

### Test 2: Get Royalty Information (Should Succeed)

```bash
# Test collection royalty info
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":200}' \
  -p $NETWORK

# Expected: ✅ Returns [500, recipient_block, recipient_tx]

# Test specific NFT royalty info (replace NFT_ID)
export NFT_ID="2:1"  # Replace with actual NFT ID from Test 1
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$NFT_ID'", "opcode":89}' \
  -p $NETWORK

# Expected: ✅ Returns royalty configuration
```

### Test 3: Transfer with Proper Royalty (Should Succeed)

```bash
# Transfer NFT with 5% royalty payment
# Sale price: 50,000 sats (0.0005 BTC)
# Required royalty: 2,500 sats (5%)

oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$NFT_ID'",
    "opcode":88,
    "data":"50000",
    "incoming_alkanes": [
      {"id":"'$NFT_ID'", "value":1},
      {"id":"0:0", "value":2500}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ Transfer succeeds, royalty paid to collection
```

### Test 4: Transfer with Insufficient Royalty (Should Fail Safely)

```bash
# Try transfer with only 1000 sats royalty instead of 2500
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$NFT_ID'",
    "opcode":88,
    "data":"50000",
    "incoming_alkanes": [
      {"id":"'$NFT_ID'", "value":1},
      {"id":"0:0", "value":1000}
    ]
  }' \
  -p $NETWORK

# Expected: ❌ "Insufficient royalty payment: 2500 sats required, 1000 provided"
# Expected: 💰 Buyer keeps their 1000 sats (transaction fails safely)
```

### Test 5: Attempt Direct Transfer (Should Fail)

```bash
# Try to use non-royalty transfer method
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$NFT_ID'",
    "opcode":0,
    "incoming_alkanes": [
      {"id":"'$NFT_ID'", "value":1}
    ]
  }' \
  -p $NETWORK

# Expected: ❌ Transaction fails or has no effect
# Expected: 🛡️ NFT remains with original owner
```

### Test 6: Minimum Royalty Enforcement

```bash
# Test with very low sale price (under minimum)
# Sale price: 5,000 sats
# Calculated royalty: 250 sats (5%)
# Minimum royalty: 1,000 sats (enforced)

oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$NFT_ID'",
    "opcode":88,
    "data":"5000",
    "incoming_alkanes": [
      {"id":"'$NFT_ID'", "value":1},
      {"id":"0:0", "value":1000}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ Transfer succeeds with 1000 sat minimum royalty
```

---

## 🏪 Marketplace Integration

### Required Implementation for Marketplaces

```javascript
class RoyaltyNFTMarketplace {
  async transferNFT(nftId, salePrice, buyer, seller) {
    // 1. Get royalty requirements
    const royaltyInfo = await this.getRoyaltyInfo(nftId);
    const royaltyPercentage = this.bytesToInt(royaltyInfo.data.slice(0, 16));
    
    // 2. Calculate royalty (minimum 1000 sats)
    const calculatedRoyalty = (salePrice * royaltyPercentage) / 10000;
    const royaltyAmount = Math.max(calculatedRoyalty, 1000);
    
    // 3. Create transfer transaction
    const transaction = {
      alkane_id: nftId,
      opcode: 88, // TransferWithRoyalty - MANDATORY
      data: this.encodeInt(salePrice),
      incoming_alkanes: [
        { id: nftId, value: 1 }, // The NFT
        { id: "0:0", value: royaltyAmount } // BTC royalty payment
      ]
    };
    
    // 4. Execute transfer
    return await this.executeTransaction(transaction);
  }
  
  async getRoyaltyInfo(nftId) {
    return await oyl.call({
      alkane_id: nftId,
      opcode: 89,
      data: ""
    });
  }
}
```

### Integration Requirements
- ✅ **Must use opcode 88** (`TransferWithRoyalty`)
- ✅ **Must include BTC payment** for royalty
- ✅ **Must calculate 5% minimum 1000 sats**
- ✅ **Handle failure gracefully** (buyer keeps money)

---

## 🐛 Troubleshooting

### Common Issues

#### "Contract not found"
```bash
# Verify contract deployment
oyl provider alkanes --method info -params '{"alkane_id":"YOUR_CONTRACT_ID"}' -p regtest
```

#### "Insufficient payment"
- Check that royalty calculation includes minimum 1000 sats
- Verify BTC token ID is correct (usually "0:0")
- Ensure total payment >= required royalty

#### "NFT not owned"
- Verify you own exactly 1 unit of the NFT
- Check that NFT ID is correct format (block:tx)

#### Template ID mismatch
- Ensure collection contract uses correct template ID from child deployment
- Rebuild collection contract after updating template ID

### Debug Commands

```bash
# Check contract state
oyl provider alkanes --method trace -params '{"txid":"TX_ID", "vout":0}' -p regtest

# Verify contract compilation
cargo check --target wasm32-unknown-unknown

# Check fuel costs
oyl provider alkanes --method simulate -params '{"alkane_id":"ID", "opcode":88}' -p regtest
```

---

## 📊 Expected Outcomes

### Successful Deployment Checklist
- [ ] ✅ Child contract deployed and returns template ID
- [ ] ✅ Collection contract deployed with correct template reference  
- [ ] ✅ NFT minting works with 0.0001 BTC payment
- [ ] ✅ Royalty transfers work with proper payment
- [ ] ✅ Insufficient royalty transfers fail safely
- [ ] ✅ Direct transfers are blocked/ineffective
- [ ] ✅ Royalties are paid to configured recipient

### Payment Flow Verification
- [ ] 💰 Primary mints: 0.0001 BTC → Collection contract
- [ ] 💰 Secondary sales: 5% royalty → Collection contract  
- [ ] 💰 Failed transactions: Buyer keeps money
- [ ] 🔒 NFT safety: No assets lost in failed transfers

---

## 🎯 Success Metrics

### Technical Success
- **Unavoidable Royalties**: ✅ All secondary sales pay 5%
- **Safe Failures**: ✅ Failed transactions don't lose assets
- **Marketplace Enforcement**: ✅ Non-compliant platforms can't complete transfers

### Economic Success  
- **Creator Revenue**: ✅ Guaranteed royalty stream
- **Market Integrity**: ✅ Level playing field for all marketplaces
- **User Safety**: ✅ Buyers and sellers protected from losses

---

## 📞 Support

For issues or questions:
1. Check contract compilation with `cargo check`
2. Verify network connectivity with `oyl provider info`  
3. Test with regtest before mainnet deployment
4. Review transaction traces for debugging

**⚠️ Important**: Always test thoroughly on regtest before mainnet deployment!

---

## 🎉 Congratulations!

You now have the first truly **unavoidable royalty system** on Bitcoin through Alkanes! 

**Key Achievement**: Secondary sales without 5% royalty payments will fail, making creator royalties genuinely unavoidable while keeping all user assets safe. 