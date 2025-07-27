# 🔍 PRE-DEPLOYMENT RESEARCH: Critical Bitcoin Address Questions

**STOP!** Before deploying the royalty system, we must answer these critical questions about how alkanes handles Bitcoin payments.

## ❓ **Critical Questions to Research**

### **Question 1: Bitcoin Address Payments**
- ❓ Can alkanes send BTC directly to Bitcoin addresses (e.g., `bc1q...`)?
- ❓ Or can alkanes only send to other alkanes (`AlkaneId`)?
- ❓ Does `CallResponse` support external Bitcoin outputs?

### **Question 2: BTC Token Nature**
```rust
const PAYMENT_TOKEN_ID: AlkaneId = AlkaneId { block: 0, tx: 0 }; // What is this exactly?
```
- ❓ Is this **native Bitcoin** or **alkane-wrapped BTC**?
- ❓ Can payments with this token go to Bitcoin addresses?
- ❓ Or do they stay within the alkanes ecosystem?

### **Question 3: PSBT Compatibility**
- ❓ When someone trades NFTs via PSBT, where can royalties be sent?
- ❓ Can royalties be included as Bitcoin outputs in the PSBT?
- ❓ Or must royalties be handled separately within alkanes?

---

## 🧪 **Research Methods**

### **Method 1: Alkanes Documentation Review**
- [ ] Check alkanes-rs documentation for Bitcoin address support
- [ ] Look for examples of sending to external addresses
- [ ] Review `CallResponse` structure for Bitcoin outputs

### **Method 2: Code Analysis**
- [ ] Examine alkanes runtime for external payment methods
- [ ] Check if `AlkaneTransfer` supports Bitcoin addresses
- [ ] Look for Bitcoin address validation in alkanes code

### **Method 3: Community Research**
- [ ] Ask in alkanes Discord/community channels
- [ ] Check existing alkanes projects for Bitcoin address usage
- [ ] Contact alkanes developers directly

### **Method 4: Simple Test Contract**
Create a minimal test to answer the core question:

```rust
// Test contract: Can we send BTC to a Bitcoin address?
fn test_bitcoin_address_payment(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    
    // Can we do something like this?
    response.bitcoin_outputs.push(BitcoinOutput {
        address: "bc1qtest...", // Regular Bitcoin address
        amount: 1000, // 1000 sats
    });
    
    // Or are we limited to this?
    response.alkanes.0.push(AlkaneTransfer {
        id: AlkaneId { block: 2, tx: 123 }, // Only alkane IDs
        value: 1000,
    });
    
    Ok(response)
}
```

---

## 🎯 **Decision Tree Based on Research Results**

### **Scenario A: Alkanes CAN send to Bitcoin addresses**
✅ **Good News**: Royalties can go directly to your Bitcoin address
- Update contracts to use Bitcoin addresses instead of AlkaneId
- Royalties are paid in real Bitcoin
- PSBTs can include royalty payments as Bitcoin outputs

### **Scenario B: Alkanes can ONLY send to other alkanes**
⚠️ **Challenge**: Need intermediate solution
- Royalties go to a "royalty collection" alkane contract
- You withdraw Bitcoin from the collection contract later
- More complex but still functional

### **Scenario C: Mixed system**
🔄 **Hybrid**: Some payments can go to addresses, others can't
- Primary mints might go to Bitcoin addresses
- Secondary royalties might need to stay in alkanes
- Need different approaches for different payment types

---

## 📋 **Research Priority Actions**

### **Immediate Actions (Next 1-2 hours)**
1. [ ] **Check alkanes documentation** for Bitcoin address examples
2. [ ] **Review existing alkanes projects** for payment patterns
3. [ ] **Ask in alkanes community** about Bitcoin address support

### **If Documentation is Unclear (Next 2-4 hours)**
4. [ ] **Create simple test contract** to verify Bitcoin address payments
5. [ ] **Deploy test on regtest** to verify functionality
6. [ ] **Test both scenarios**: alkane payments vs Bitcoin address payments

### **If Still Unclear (Next 24 hours)**
7. [ ] **Contact alkanes developers** directly for clarification
8. [ ] **Review alkanes runtime source code** for external payment support
9. [ ] **Consider alternative architectures** based on limitations

---

## 🚨 **DEPLOYMENT BLOCKER**

**DO NOT PROCEED WITH DEPLOYMENT** until we have clear answers to these questions.

**Why this matters:**
- ❌ **Wrong assumption**: Could build unusable royalty system
- 💰 **Payment issues**: Users might not receive Bitcoin as expected  
- 🔄 **Architecture changes**: May need completely different approach
- ⏰ **Wasted time**: Could avoid rebuilding later

---

## 📞 **Next Steps**

1. **Start research immediately** using methods above
2. **Document findings** in this file as you discover them
3. **Update architecture** based on research results
4. **Only then proceed** with deployment checklists

**Remember**: It's better to spend a few hours researching now than to deploy a broken royalty system! 