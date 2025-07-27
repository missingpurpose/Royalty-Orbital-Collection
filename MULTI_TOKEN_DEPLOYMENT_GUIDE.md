# Multi-Token NFT Collection Deployment Guide

## Overview

This NFT collection contract supports **multiple payment tokens** (frBTC and BUSD) with different pricing. Here's the complete deployment process.

## Deployment Sequence

### For Regtest

**Step 1: Deploy Payment Tokens First**
```bash
# Deploy frBTC (wrapped BTC) token
alkanes-cli deploy-token --name "Fractional BTC" --symbol "frBTC" --supply 1000000000000

# Deploy BUSD (stablecoin) token  
alkanes-cli deploy-token --name "Bitcoin USD" --symbol "BUSD" --supply 1000000000000
```

**Step 2: Note the AlkaneIds**
After deployment, you'll get AlkaneIds like:
```
frBTC: AlkaneId { block: 3, tx: 1001 }
BUSD:  AlkaneId { block: 3, tx: 1002 }
```

**Step 3: Update Your NFT Contract**
In `src/lib.rs`, update these constants:
```rust
/// UPDATE THESE WITH YOUR ACTUAL DEPLOYED TOKEN IDs
const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 3, tx: 1001 }; // From Step 2
const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 3, tx: 1002 };  // From Step 2

/// Adjust pricing as needed
const FRBTC_AMOUNT_PER_MINT: u128 = 10000;    // 0.0001 BTC equivalent
const BUSD_AMOUNT_PER_MINT: u128 = 1000000;   // $10 in BUSD (6 decimals)
```

**Step 4: Deploy NFT Collection**
```bash
alkanes-cli deploy --contract-path ./target/wasm32-unknown-unknown/release/alkane_pandas_collection.wasm
```

### For Mainnet

**Step 1: Find Existing Token IDs**
Research the mainnet to find the AlkaneIds of:
- frBTC (or other wrapped BTC token)
- BUSD (or your preferred stablecoin)

Example mainnet IDs (these are examples - verify actual IDs):
```rust
const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 2, tx: 9000 };  // Actual frBTC ID
const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 2, tx: 9001 };   // Actual BUSD ID
```

**Step 2: Update Contract & Deploy**
Same as regtest steps 3-4, but with mainnet token IDs.

## Contract Features

### Multi-Token Payment System
- **frBTC**: Bitcoin-equivalent payments
- **BUSD**: Stablecoin payments for consistent USD pricing
- **Automatic calculation**: Users can pay with either token type
- **Mixed payments**: Can even pay with both tokens in one transaction

### Pricing Configuration
```rust
// Example pricing (adjust for your use case)
const FRBTC_AMOUNT_PER_MINT: u128 = 10000;    // 0.0001 BTC
const BUSD_AMOUNT_PER_MINT: u128 = 1000000;   // $10 BUSD (6 decimals)
```

### Usage Examples

**Minting with frBTC:**
```bash
alkanes-cli call --contract [NFT_CONTRACT_ID] --method mint_orbital --payment "frBTC:10000"
```

**Minting with BUSD:**
```bash
alkanes-cli call --contract [NFT_CONTRACT_ID] --method mint_orbital --payment "BUSD:1000000"
```

**Minting with mixed tokens:**
```bash
alkanes-cli call --contract [NFT_CONTRACT_ID] --method mint_orbital --payment "frBTC:5000,BUSD:500000"
# This would mint 1 NFT (0.5 from frBTC + 0.5 from BUSD)
```

## New Contract Methods

### `get_accepted_tokens()` - Opcode 202
Returns the AlkaneIds of accepted payment tokens:
```
Returns: [frBTC_block(16), frBTC_tx(16), BUSD_block(16), BUSD_tx(16)]
```

### `get_token_prices()` - Opcode 203  
Returns the price per mint for each token:
```
Returns: [frBTC_price(16), BUSD_price(16)]
```

### `withdraw_funds(token_type, amount)` - Opcode 201
Withdraw accumulated funds by token type:
- `token_type`: 0 = frBTC, 1 = BUSD
- `amount`: Amount to withdraw

## Directory Structure
```
alkane-pandas-collection-MY FORK/
├── src/
│   ├── lib.rs                          # Main contract (UPDATE TOKEN IDs HERE)
│   ├── svg_generator.rs               # NFT art generation
│   ├── encoded_traits.json            # Trait definitions
│   └── svg-templates.json             # Art templates
├── Cargo.toml                         # Dependencies
├── build.rs                           # Build script
└── MULTI_TOKEN_DEPLOYMENT_GUIDE.md    # This guide
```

## Security Considerations

1. **Token Validation**: Only pre-approved tokens can be used for payment
2. **Owner-Only Functions**: Withdrawals require collection token authentication
3. **Overflow Protection**: All arithmetic operations use checked math
4. **Price Flexibility**: Different pricing for different tokens allows market adaptation

## Testing Commands

Run these commands in the project root directory (`/Volumes/btc-node/everything-alkanes/My-Contracts/alkane-pandas-collection-MY%20FORK/`):

```bash
# Build the contract
cargo build --release --target wasm32-unknown-unknown

# Test locally (after updating token IDs)
alkanes-cli test --contract ./target/wasm32-unknown-unknown/release/alkane_pandas_collection.wasm

# Deploy to regtest (after configuring regtest token IDs)
alkanes-cli deploy --network regtest --contract ./target/wasm32-unknown-unknown/release/alkane_pandas_collection.wasm

# Deploy to mainnet (after configuring mainnet token IDs)
alkanes-cli deploy --network mainnet --contract ./target/wasm32-unknown-unknown/release/alkane_pandas_collection.wasm
```

## Important Notes

- **Always deploy payment tokens BEFORE the NFT collection**
- **Update token IDs in the contract BEFORE deployment**
- **Test thoroughly on regtest before mainnet deployment**
- **Verify token IDs are correct for your target network**
- **Consider token decimal places when setting prices**

## Why Not `block: 0, tx: 0`?

The `AlkaneId { block: 0, tx: 0 }` was a placeholder. Real alkanes get assigned actual block/transaction numbers when deployed. Using placeholder values would cause:
- Payment verification failures
- Users unable to mint (no valid payment token)
- Contract essentially unusable

Always use real, deployed token IDs! 