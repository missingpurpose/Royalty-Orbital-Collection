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

## 💰 Payment System Overview

### Understanding Alkanes BTC Payments

**⚠️ IMPORTANT**: This system uses **alkanes BTC** (wrapped Bitcoin), not regular Bitcoin addresses.

### Multi-Token Payment Configuration
```rust
// Update these for your target network:
const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 };    // Regtest frBTC
const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 };     // Regtest BUSD

// For mainnet:
// const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 2, tx: 9000 };  // Actual frBTC ID
// const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 3, tx: 1500 };   // Actual BUSD ID

// Payment amounts per token type:
const FRBTC_AMOUNT_PER_MINT: u128 = 10000;      // 0.0001 BTC equivalent
const BUSD_AMOUNT_PER_MINT: u128 = 1000000;     // $10 in BUSD (6 decimals)
```

### Enhanced Multi-Token Revenue Flow
1. **Primary Sales**: Users pay with frBTC (0.0001) OR BUSD ($10) → Collection Contract
2. **Mixed Payments**: Users can combine tokens in single transaction for multiple NFTs
3. **Secondary Sales**: 5% royalty (in any supported token) → Collection Contract  
4. **Token-Specific Balances**: Each token type accumulates separately in contract
5. **Flexible Withdrawal**: Extract earnings by specific token type (opcode 201)

### What You Receive
- ✅ **Real Bitcoin value** within alkanes ecosystem
- ✅ **Guaranteed collection** through unavoidable royalties
- ✅ **Owner-only withdrawal** for security
- ❌ **Not direct Bitcoin** to external addresses (alkanes limitation)

---

## 🏗️ Architecture Overview

### System Components
1. **Collection Contract** (`alkane-pandas-collection-MY FORK/`)
   - Factory that mints NFTs
   - Configures royalty settings
   - **Accumulates all payments** (primary mints + royalties)
   - **Enables owner withdrawal** of accumulated funds

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

### Step 1: Configure Payment Token ID

**🚨 CRITICAL**: Update the payment token IDs for your target network.

Edit `alkane-pandas-collection-MY FORK/src/lib.rs`:

```rust
/// Multi-token payment configuration - UPDATE FOR YOUR NETWORK
const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 };    // Deploy frBTC first
const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 };     // Deploy BUSD first

/// Payment amounts per token type (adjust based on current values)
const FRBTC_AMOUNT_PER_MINT: u128 = 10000;      // 0.0001 BTC equivalent
const BUSD_AMOUNT_PER_MINT: u128 = 1000000;     // $10 in BUSD
```

**How to find the correct AlkaneIds**:
1. **Deploy tokens first**: Deploy frBTC and BUSD contracts on your network
2. **Record AlkaneIds**: Note the `block:tx` format for each deployed token
3. **Update constants**: Replace the placeholder IDs with actual deployed IDs
4. **Adjust pricing**: Set appropriate amounts based on current token values

### Step 2: Configure Royalty Recipients

The royalty recipients are automatically set to the Collection Contract:

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

### Test 1: Mint NFT with frBTC (Should Succeed)

```bash
# Mint with frBTC payment (0.0001 BTC equivalent)
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
```

### Test 1b: Mint NFT with BUSD (Should Succeed)

```bash
# Mint with BUSD payment ($10 equivalent)
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'", 
    "opcode":77,
    "incoming_alkanes": [
      {"id":"0:1", "value":1000000}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ Success with new NFT created
# Note the NFT ID from response for transfer tests
```

### Test 1c: Multi-Token Batch Mint (Should Succeed)

```bash
# Mint 2 NFTs: 1 with frBTC + 1 with BUSD in single transaction
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'", 
    "opcode":77,
    "incoming_alkanes": [
      {"id":"0:0", "value":10000},
      {"id":"0:1", "value":1000000}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ Success with 2 NFTs created from mixed payment
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

### Test 7: Query Accepted Tokens (Should Succeed)

```bash
# Get list of accepted payment tokens (opcode 202)
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":202}' \
  -p $NETWORK

# Expected: ✅ Returns [frBTC_block, frBTC_tx, BUSD_block, BUSD_tx]
```

### Test 8: Query Token Prices (Should Succeed)

```bash
# Get current prices for each token (opcode 203)
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":203}' \
  -p $NETWORK

# Expected: ✅ Returns [frBTC_price, BUSD_price]
```

### Test 9: Withdraw frBTC Funds (Owner Only)

```bash
# Withdraw accumulated frBTC from collection contract
# Parameters: [token_type (0=frBTC), amount]
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'",
    "opcode":201,
    "data":"0,50000",
    "incoming_alkanes": [
      {"id":"'$COLLECTION_ID'", "value":1}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ 50,000 sats of frBTC withdrawn to your wallet
```

### Test 10: Withdraw BUSD Funds (Owner Only)

```bash
# Withdraw accumulated BUSD from collection contract  
# Parameters: [token_type (1=BUSD), amount]
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'",
    "opcode":201,
    "data":"1,500000",
    "incoming_alkanes": [
      {"id":"'$COLLECTION_ID'", "value":1}
    ]
  }' \
  -p $NETWORK

# Expected: ✅ 500,000 units of BUSD withdrawn to your wallet
```

### Test 11: Unauthorized Withdrawal (Should Fail)

```bash
# Test withdrawal without collection token authentication
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'",
    "opcode":201,
    "data":"0,10000",
    "incoming_alkanes": []
  }' \
  -p $NETWORK

# Expected: ❌ Fails with "did not authenticate with only the collection token"
```

---

## 💸 Revenue Withdrawal Guide

### How Withdrawals Work

Your Collection Contract accumulates **all revenue by token type**:
- ✅ **Primary mints**: frBTC (0.0001) OR BUSD ($10) per NFT minted
- ✅ **Secondary royalties**: 5% of every trade (in any supported token)
- ✅ **Separate balances**: Each token type stored independently  
- ✅ **Secure storage**: Only you can withdraw (owner-only)

### Enhanced Withdrawal Function Details

```rust
// Opcode 201: WithdrawFunds - Multi-token support
fn withdraw_funds(&self, token_type: u128, amount: u128) -> Result<CallResponse> {
  self.only_owner()?; // Requires collection token authentication
  // token_type: 0=frBTC, 1=BUSD
  // amount: Specific amount of chosen token to withdraw
}

// Opcode 202: GetAcceptedTokens - Query supported tokens
fn get_accepted_tokens(&self) -> Result<CallResponse> {
  // Returns: [frBTC_block, frBTC_tx, BUSD_block, BUSD_tx]
}

// Opcode 203: GetTokenPrices - Query current pricing  
fn get_token_prices(&self) -> Result<CallResponse> {
  // Returns: [frBTC_price, BUSD_price] per mint
}
```

### Step-by-Step Withdrawal Process

#### 1. Check Your Balance (Optional)
```bash
# Query contract state to see accumulated funds
oyl provider alkanes --method info \
  -params '{"alkane_id":"'$COLLECTION_ID'"}' \
  -p $NETWORK
```

#### 2. Withdraw Funds by Token Type

**Withdraw frBTC:**
```bash
# Withdraw 100,000 sats of frBTC (0.001 BTC equivalent)
# Parameters: token_type=0 (frBTC), amount=100000
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'",
    "opcode":201,
    "data":"0,100000",
    "incoming_alkanes": [
      {"id":"'$COLLECTION_ID'", "value":1}
    ]
  }' \
  -p $NETWORK
```

**Withdraw BUSD:**
```bash  
# Withdraw 5,000,000 units of BUSD ($50 equivalent)
# Parameters: token_type=1 (BUSD), amount=5000000
oyl provider alkanes --method call \
  -params '{
    "alkane_id":"'$COLLECTION_ID'",
    "opcode":201,
    "data":"1,5000000",
    "incoming_alkanes": [
      {"id":"'$COLLECTION_ID'", "value":1}
    ]
  }' \
  -p $NETWORK
```

#### 3. Verify Withdrawal Success
- Check transaction confirmation
- Verify funds received in your wallet
- Update records for accounting

### Withdrawal Security Features

- ✅ **Owner-only**: Requires collection token authentication
- ✅ **Amount validation**: Prevents zero withdrawals
- ✅ **Safe failure**: Failed withdrawals don't lose funds
- ✅ **Full control**: Withdraw any amount, any time

### Common Withdrawal Scenarios

**Scenario 1: Weekly frBTC Revenue Collection**
```bash
# Weekly withdrawal of accumulated frBTC royalties
WEEKLY_FRBTC="250000"  # 0.0025 BTC equivalent
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":201, "data":"0,'$WEEKLY_FRBTC'", "incoming_alkanes":[{"id":"'$COLLECTION_ID'", "value":1}]}' \
  -p $NETWORK
```

**Scenario 2: Weekly BUSD Revenue Collection**
```bash
# Weekly withdrawal of accumulated BUSD royalties  
WEEKLY_BUSD="2500000"  # $25 equivalent in BUSD
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":201, "data":"1,'$WEEKLY_BUSD'", "incoming_alkanes":[{"id":"'$COLLECTION_ID'", "value":1}]}' \
  -p $NETWORK
```

**Scenario 3: Emergency Full Withdrawal (Both Tokens)**
```bash
# Withdraw large amounts of both token types
LARGE_FRBTC="1000000"   # 0.01 BTC equivalent
LARGE_BUSD="10000000"   # $100 equivalent

# Withdraw frBTC
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":201, "data":"0,'$LARGE_FRBTC'", "incoming_alkanes":[{"id":"'$COLLECTION_ID'", "value":1}]}' \
  -p $NETWORK

# Withdraw BUSD  
oyl provider alkanes --method call \
  -params '{"alkane_id":"'$COLLECTION_ID'", "opcode":201, "data":"1,'$LARGE_BUSD'", "incoming_alkanes":[{"id":"'$COLLECTION_ID'", "value":1}]}' \
  -p $NETWORK
```

### Troubleshooting Withdrawals

**Error: "did not authenticate with only the collection token"**
- ✅ **Solution**: Include collection token in `incoming_alkanes`
- ✅ **Format**: `{"id":"YOUR_COLLECTION_ID", "value":1}`

**Error: "Invalid token type. Use 0 for frBTC, 1 for BUSD"**
- ✅ **Solution**: Use correct token type parameter (0 or 1)
- ✅ **Format**: `"data":"0,amount"` for frBTC or `"data":"1,amount"` for BUSD

**Error: "Withdrawal amount must be greater than zero"**
- ✅ **Solution**: Ensure amount > 0 in the data field
- ✅ **Format**: `"data":"token_type,amount"` where amount > 0

**Error: Insufficient funds for specific token**
- ✅ **Solution**: Reduce withdrawal amount for that token type
- ✅ **Check**: Verify contract has accumulated sufficient revenue in that specific token
- ✅ **Alternative**: Try withdrawing the other token type instead

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