# ✅ PRE-REGTEST DEPLOYMENT CHECKLIST

**🎯 Purpose**: This checklist ensures everything is ready for regtest deployment and testing of your revolutionary unavoidable royalty NFT system.

**📁 Your Setup**:
- Docker Regtest: `/Volumes/btc-node/everything-alkanes/alkanes-dev-environment`
- OYL SDK: `/Volumes/btc-node/everything-alkanes/oyl-sdk-main-`
- Collection: `/Volumes/btc-node/everything-alkanes/My-Contracts/alkane-pandas-collection-MY FORK`
- Child: `/Volumes/btc-node/everything-alkanes/My-Contracts/alkane-pandas-child-main`

---

## 📋 **Pre-Deployment Checklist**

### **🔍 Research & Architecture ✅ COMPLETED**
- [x] ✅ **Completed** `PRE_DEPLOYMENT_RESEARCH.md`
- [x] ✅ **Confirmed** alkanes can only send to `AlkaneId`s (wrapped tokens)
- [x] ✅ **Updated contracts** to use Collection Contract as payment recipient
- [x] ✅ **Verified** PSBTs cannot bypass royalty system

### **🌐 Environment Setup**
- [ ] 🐳 **Docker regtest running** in `/Volumes/btc-node/everything-alkanes/alkanes-dev-environment`
- [ ] 🔧 **OYL SDK accessible** from `/Volumes/btc-node/everything-alkanes/oyl-sdk-main-`
- [ ] 🦀 **Rust installed** and updated (`rustup update`)
- [ ] 🎯 **WebAssembly target** added (`rustup target add wasm32-unknown-unknown`)
- [ ] 📡 **Network connectivity** verified (can connect to regtest)
- [ ] 💰 **Regtest Bitcoin** available for testing transactions

### **📦 Contract Build Status ✅ COMPLETED**
- [x] ✅ **Collection contract compiles** (`cargo build --release`)
- [x] ✅ **Child contract compiles** (both contracts build successfully)
- [x] ✅ **WASM builds created**:
  - Collection: `310KB → 113KB` compressed (.gz)
  - Child: `195KB → 69KB` compressed (.gz)
- [x] ✅ **Build warnings acceptable** (unused constants are non-critical)

### **🪙 Token Deployment Strategy (YOUR NEXT STEP)**

**Phase 1: Deploy Test Tokens**
- [ ] 🏗️ **Deploy test frBTC token** (free mint fungible for testing)
  - Record `AlkaneId { block: X, tx: Y }` 
  - Test minting functionality
- [ ] 🏗️ **Deploy test BUSD token** (free mint fungible for testing)
  - Record `AlkaneId { block: X, tx: Y }`
  - Test minting functionality
- [ ] 📝 **Document token IDs** for contract configuration

**Phase 2: Update Contract Configuration**
- [ ] 🔧 **Update Collection Contract Constants**:
  ```rust
  const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: X, tx: Y }; // From Phase 1
  const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: X, tx: Y };  // From Phase 1
  ```
- [ ] 💰 **Verify pricing configuration**:
  - `FRBTC_AMOUNT_PER_MINT: u128 = 10000` (0.0001 BTC equivalent)
  - `BUSD_AMOUNT_PER_MINT: u128 = 1000000` ($10 USD equivalent)
- [ ] 🔄 **Rebuild contracts** after token ID updates
- [ ] ✅ **Re-compress WASM files** after configuration updates

### **🎭 Contract Deployment Sequence**

**Step 1: Deploy Child Contract Template**
- [ ] 🎭 **Deploy child contract** using OYL SDK
- [ ] 📝 **Record template AlkaneId** (will be like `{ block: Z, tx: W }`)
- [ ] ✅ **Verify deployment successful**

**Step 2: Update & Deploy Collection Contract**
- [ ] 🔧 **Update Collection Contract Constants**:
  ```rust
  const ROYALTY_NFT_ORBITAL_TEMPLATE_ID: u128 = W; // tx from child deployment
  ```
- [ ] 🔄 **Rebuild collection contract** with correct template ID
- [ ] 🏗️ **Deploy collection contract** using OYL SDK
- [ ] 📝 **Record collection AlkaneId** for testing

### **🧪 Pre-Deployment Testing (Recommended)**

**Local Testing**
- [ ] 🧮 **Royalty calculation verified**: 5% of sale price, minimum 1000 sats
- [ ] 💳 **Multi-token payment logic tested**: frBTC OR BUSD payments
- [ ] 🎨 **Algorithmic art generation verified**: All 6 styles working
- [ ] 📊 **Batch minting logic tested**: Up to 3 NFTs per transaction

**Integration Testing**
- [ ] 🔗 **Child → Collection communication**: Opcodes 999, 1000 working
- [ ] 🎯 **Template instantiation**: Child contracts created with correct index
- [ ] 💰 **Revenue accumulation**: Payments flow to Collection Contract
- [ ] 🔒 **Royalty enforcement**: Only `TransferWithRoyalty` (opcode 88) works

### **📋 Deployment Readiness Checklist**

**Documentation Review**
- [ ] 📖 **Read** `DOCUMENTATION_GUIDE.md` - Understand file usage order
- [ ] 🚀 **Review** `ROYALTY_DEPLOYMENT_GUIDE.md` - Deployment steps
- [ ] 🪙 **Review** `MULTI_TOKEN_DEPLOYMENT_GUIDE.md` - Token setup process

**Risk Assessment**
- [ ] ⚠️ **Testnet only**: Confirm this is regtest deployment (no mainnet risk)
- [ ] 💸 **Limited exposure**: Only test tokens being used
- [ ] 🔄 **Reversible**: Can redeploy if issues found
- [ ] 📝 **Documented**: All deployment steps will be recorded

### **✅ Success Criteria**

**Before proceeding to deployment, ALL must be true**:
- [ ] 🌐 **Environment**: Docker regtest running, OYL SDK accessible
- [ ] 📦 **Builds**: Both contracts compile and compress successfully  
- [ ] 🪙 **Tokens**: Test frBTC and BUSD deployed and IDs recorded
- [ ] 🔧 **Configuration**: All AlkaneIds updated in contract constants
- [ ] 📖 **Documentation**: Deployment guides reviewed and understood
- [ ] 🧪 **Testing**: Basic functionality verified locally

---

## 🚀 **Ready for Deployment!**

Once ALL checkboxes above are completed:

1. **📖 Follow**: `ROYALTY_DEPLOYMENT_GUIDE.md` for step-by-step deployment
2. **🪙 Start with**: Token deployment (frBTC and BUSD)
3. **🎭 Then deploy**: Child contract template
4. **🏭 Finally deploy**: Collection contract with all correct IDs

**🎊 You're about to make Bitcoin NFT history!** 🎊

---

## 📁 **File Locations Reference**

```
📂 Your Setup:
├── 🐳 alkanes-dev-environment/     # Docker regtest
├── 🔧 oyl-sdk-main-/              # Deployment tool
├── 🏭 alkane-pandas-collection-MY FORK/
│   ├── target/wasm32-unknown-unknown/release/
│   │   └── alkane_pandas.wasm.gz   # 113KB compressed
│   └── [All .md documentation files]
└── 🎭 alkane-pandas-child-main/
    └── target/wasm32-unknown-unknown/release/
        └── alkane_pandas_child.wasm.gz  # 69KB compressed
```

**🎯 Next Action**: Deploy test tokens (frBTC and BUSD) first, then update contract configurations! 