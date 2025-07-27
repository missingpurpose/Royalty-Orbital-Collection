# 📖 **Documentation Guide: How to Use These Files**

This guide explains what each documentation file is for and how you should use them to successfully deploy your unavoidable royalty NFT system.

---

## 📋 **File Overview & Usage Order**

### **🔍 1. PRE_DEPLOYMENT_RESEARCH.md**
**Purpose**: ✅ **COMPLETED** - Research findings on alkanes architecture limitations

**What it contains**:
- ✅ Confirmed that alkanes cannot send to external Bitcoin addresses
- ✅ Confirmed that PSBTs cannot bypass the royalty system
- ✅ Architecture decision: Use Collection Contract as payment recipient
- ✅ Multi-token payment system explanation

**How to use**:
- ✅ **Already complete** - no action needed
- 📖 **Reference only** - understand the architectural decisions made
- 🎯 **Key takeaway**: Your system uses alkanes wrapped tokens, not direct Bitcoin

---

### **📋 2. PRE_REGTEST_CHECKLIST.md** 
**Purpose**: 🔄 **USE THIS NEXT** - Prepare for regtest deployment and testing

**What it contains**:
- Environment setup verification (Rust, OYL SDK, regtest)
- Contract compilation checks
- Configuration verification
- Pre-deployment risk assessment
- Success criteria definition

**How to use**:
1. 📋 **Work through each checklist item** systematically
2. ✅ **Check off completed items** as you go
3. 🚫 **Do not proceed** if any high-risk items fail
4. 🎯 **Only move to deployment** when ALL items complete

**When to use**: Before your first regtest deployment

---

### **🚀 3. ROYALTY_DEPLOYMENT_GUIDE.md**
**Purpose**: 📖 **MAIN DEPLOYMENT GUIDE** - Step-by-step deployment and testing

**What it contains**:
- Multi-token payment system overview
- Configuration instructions (token IDs, pricing)
- Detailed deployment steps (child first, then collection)
- 11 comprehensive test scenarios
- Revenue withdrawal guide
- Marketplace integration examples
- Troubleshooting section

**How to use**:
1. 🔧 **Configure token IDs** for your network (Step 1)
2. 🏗️ **Follow deployment steps** in exact order
3. 🧪 **Run all 11 tests** to verify functionality
4. 💰 **Practice withdrawal** procedures
5. 📚 **Reference troubleshooting** if issues arise

**When to use**: During actual deployment and testing phases

---

### **🚨 4. PRE_MAINNET_CHECKLIST.md**
**Purpose**: 🔒 **MAINNET SAFETY** - Comprehensive pre-production checklist

**What it contains**:
- Mandatory prerequisites (48+ hours regtest success)
- Security audit requirements
- Comprehensive testing verification
- Financial preparation (costs, addresses)
- Risk management procedures
- Final authorization process

**How to use**:
1. ⏰ **Only use AFTER** successful regtest deployment
2. 📋 **Complete every single item** - no exceptions
3. 👥 **Get team approval** for high-risk sections
4. 🔒 **Triple-check** all configurations
5. 💰 **Verify financial** setups and recipient addresses

**When to use**: Before mainnet deployment (after regtest success)

---

## 🎯 **Recommended Usage Workflow**

### **Phase 1: Understanding (Day 1)**
1. 📖 **Read PRE_DEPLOYMENT_RESEARCH.md** - Understand architecture decisions
2. 📖 **Skim ROYALTY_DEPLOYMENT_GUIDE.md** - Get overview of deployment process
3. 🎯 **Understand** multi-token payment system (frBTC + BUSD)

### **Phase 2: Preparation (Days 2-3)**
1. 📋 **Work through PRE_REGTEST_CHECKLIST.md** systematically
2. 🔧 **Set up environment** (Rust, OYL SDK, regtest)
3. 🎯 **Deploy token contracts** (frBTC and BUSD) first
4. 📝 **Update token IDs** in collection contract

### **Phase 3: Regtest Deployment (Days 4-7)**
1. 🚀 **Follow ROYALTY_DEPLOYMENT_GUIDE.md** exactly
2. 🧪 **Run all 11 test scenarios**
3. 💰 **Practice withdrawal** procedures
4. 🔄 **Repeat until perfect** (multiple clean deployments)

### **Phase 4: Mainnet Preparation (Days 8-14)**
1. ⏰ **Wait minimum 48 hours** of regtest stability
2. 📋 **Complete PRE_MAINNET_CHECKLIST.md** entirely
3. 🔒 **Security audit** and team review
4. 💰 **Financial preparation** and address verification

### **Phase 5: Mainnet Deployment (Day 15+)**
1. 🚀 **Deploy to mainnet** following same process
2. 🧪 **Run verification tests** immediately
3. 📊 **Monitor for 24-48 hours**
4. 🎉 **Launch your unavoidable royalty system!**

---

## 🎯 **Key Configuration Requirements**

### **Token Deployment Order**
```bash
# 1. Deploy frBTC contract → Get AlkaneId (e.g., 2:1000)
# 2. Deploy BUSD contract → Get AlkaneId (e.g., 2:1001) 
# 3. Update collection contract constants:
const FRBTC_TOKEN_ID: AlkaneId = AlkaneId { block: 2, tx: 1000 };
const BUSD_TOKEN_ID: AlkaneId = AlkaneId { block: 2, tx: 1001 };
```

### **Critical Configuration Points**
- ✅ **Token IDs must match deployed contracts** (not placeholder values)
- ✅ **Pricing must reflect current token values** 
- ✅ **Template ID must match deployed child contract**
- ✅ **All constants updated before deployment**

---

## 🚨 **Important Notes**

### **⚠️ What NOT to Skip**
- 🚫 **Never skip PRE_REGTEST_CHECKLIST.md** - prevents major issues
- 🚫 **Never skip testing** - all 11 tests must pass
- 🚫 **Never rush to mainnet** - 48+ hour regtest minimum
- 🚫 **Never deploy without token contracts first**

### **✅ Success Indicators**
- 🎯 **All tests pass** without any errors
- 💰 **Withdrawal functions** work for both tokens
- 🔒 **Royalty enforcement** cannot be bypassed
- 🏪 **Marketplace integration** examples work

### **🆘 When to Get Help**
- ❌ **Any checklist item fails** repeatedly
- ❌ **Tests don't pass** after multiple attempts
- ❌ **Contract compilation errors** you can't resolve
- ❌ **Token deployment issues**

---

## 🎉 **What You're Building**

**The World's First Truly Unavoidable Bitcoin NFT Royalty System**

### **Revolutionary Features**:
- 🔒 **100% unavoidable** - no bypass methods exist
- 💰 **Multi-token support** - frBTC and BUSD payments
- 🛡️ **Safe failures** - no asset loss in failed transactions
- 🏪 **Marketplace enforcement** - platforms must comply
- 💸 **Flexible withdrawals** - by token type
- 🎯 **Owner-only revenue** - secure and private

### **Technical Achievement**:
- ✅ **PSBTs cannot bypass** the system
- ✅ **Direct transfers blocked** - only royalty transfers work
- ✅ **5% minimum royalty** with 1000 sat floor
- ✅ **Batch minting support** up to 3 NFTs per transaction
- ✅ **Dynamic pricing** based on token type

---

## 📞 **Ready to Start?**

1. 📖 **Read PRE_DEPLOYMENT_RESEARCH.md** first (just for understanding)
2. 📋 **Open PRE_REGTEST_CHECKLIST.md** and start working through it
3. 🚀 **Deploy tokens first**, then update your constants
4. 🎯 **Follow ROYALTY_DEPLOYMENT_GUIDE.md** for actual deployment

**You're about to make Bitcoin NFT history!** 🚀 