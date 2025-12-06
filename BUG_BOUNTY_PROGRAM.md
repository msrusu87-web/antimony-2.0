# ANTIMONY 2.0 - Bug Bounty Program

## Overview

Welcome to the Antimony Coin Bug Bounty Program! We're committed to the security of our blockchain and reward researchers who help us maintain the highest security standards.

**Program Status:** 🟢 ACTIVE  
**Platform:** HackerOne  
**Total Rewards Pool:** 100,000 ATMN (~$50,000 USD)  
**Program Start:** December 7, 2025

---

## Scope

### In-Scope Assets

**Blockchain Core:**
- Consensus engine (SHA-256d)
- Transaction validation
- UTXO management
- Difficulty adjustment
- Block propagation
- P2P networking

**APIs:**
- Rosetta API (Data & Construction)
- Mining Pool API
- Wallet API
- Block Explorer API

**Infrastructure:**
- Web Wallet (https://explorer.carphatian.ro/web-wallet.html)
- Mining Pool (https://miningpool.carphatian.ro)
- Block Explorer (https://explorer.carphatian.ro)

**Source Code:**
- Repository: https://github.com/msrusu87-web/antimony-2.0
- Branches: main, develop

---

## Reward Tiers

### 🔴 CRITICAL (P1)
**Reward: 10,000 - 50,000 ATMN**

- Private key extraction
- Consensus bypass (double-spend, 51% attack)
- Remote code execution (RCE)
- Unauthorized minting of coins
- Blockchain state manipulation
- Critical cryptographic flaws

### 🟠 HIGH (P2)
**Reward: 5,000 - 10,000 ATMN**

- Transaction replay attacks
- P2P network DoS (persistent)
- Wallet private key leakage
- Mining pool reward manipulation
- SQL injection leading to data breach
- Authentication bypass (admin access)

### 🟡 MEDIUM (P3)
**Reward: 1,000 - 5,000 ATMN**

- Information disclosure (sensitive data)
- CSRF attacks
- XSS (stored/reflected)
- API rate limit bypass
- Temporary DoS (< 1 hour recovery)
- Logic flaws in fee calculation

### 🟢 LOW (P4)
**Reward: 500 - 1,000 ATMN**

- Minor information disclosure
- SPF/DKIM misconfigurations
- SSL/TLS configuration issues
- Clickjacking
- Open redirects
- Content injection

---

## Out of Scope

❌ **Not Eligible for Rewards:**

- Theoretical vulnerabilities without working PoC
- Social engineering attacks
- Physical attacks
- DoS attacks without persistent impact
- Spam/flooding without damage
- Known issues already reported
- Issues in third-party dependencies (unless directly exploitable)
- Attacks requiring physical access
- Browser-specific issues (IE, old browsers)
- Rate limiting on non-critical endpoints
- Missing HTTP security headers (without demonstrated impact)

---

## Submission Guidelines

### Required Information

1. **Vulnerability Description**
   - Clear explanation of the issue
   - Attack scenario
   - Potential impact

2. **Proof of Concept (PoC)**
   - Step-by-step reproduction
   - Screenshots/videos
   - Test accounts used
   - Network traffic (if applicable)

3. **Environment**
   - Testnet or Mainnet
   - Software versions
   - Operating system
   - Browser (for web vulnerabilities)

4. **Remediation Suggestions**
   - Recommended fixes
   - Code patches (optional)
   - References to best practices

### Submission Process

1. **Report via HackerOne:**
   https://hackerone.com/antimony (Coming Soon)

2. **Or Email:**
   security@antimony.network
   - Use PGP key for sensitive information
   - Subject: [BUG BOUNTY] Brief Description

3. **Wait for Initial Response:**
   - Acknowledgment within 48 hours
   - Triage within 5 business days
   - Regular updates on progress

### Rules

✅ **DO:**
- Test on testnet whenever possible
- Respect user privacy
- Report vulnerabilities promptly
- Provide detailed, reproducible PoCs
- Allow reasonable time for fixes (90 days)
- Be professional and courteous

❌ **DON'T:**
- Test on mainnet without permission
- Exploit vulnerabilities beyond PoC
- Access, modify, or delete user data
- Disrupt services
- Publicly disclose before resolution
- Perform social engineering
- Use automated scanners without approval

---

## Evaluation Criteria

Bounties are awarded based on:

1. **Severity** (CVSS 3.1 Score)
2. **Impact** (users affected, data at risk)
3. **Exploitability** (ease of exploitation)
4. **Quality of Report** (clarity, completeness)
5. **Novelty** (previously unknown vs. known issue)

### CVSS Scoring

- **Critical:** 9.0 - 10.0
- **High:** 7.0 - 8.9
- **Medium:** 4.0 - 6.9
- **Low:** 0.1 - 3.9

---

## Payment Process

### How You'll Be Paid

1. **Bounty Approval**
   - Security team validates finding
   - Severity and reward determined
   - Researcher notified

2. **ATMN Address Submission**
   - Provide valid ATMN address
   - Verify ownership via signed message

3. **Payment**
   - Sent within 7 business days of approval
   - On-chain transaction
   - Confirmation sent via email

4. **Public Recognition** (Optional)
   - Added to Security Hall of Fame
   - Name/handle/anonymous

### Exchange Rate

ATMN rewards are calculated at time of bounty approval based on 7-day moving average from:
- CoinGecko API
- CoinMarketCap API
- Internal exchange rates

Minimum: $0.50 per ATMN  
Maximum: $5.00 per ATMN

---

## Timeline Expectations

| Stage | Expected Time |
|-------|---------------|
| Initial Response | 48 hours |
| Triage | 5 business days |
| Fix Development | 30-90 days |
| Fix Deployment | 7 days after fix |
| Bounty Payment | 7 days after approval |

---

## Safe Harbor

We consider security research conducted under this program to be:
- Authorized under the Computer Fraud and Abuse Act (CFAA)
- Authorized under applicable anti-hacking laws
- Compliant with our Terms of Service for research purposes

We will not pursue legal action against researchers who:
- Follow the rules and guidelines
- Act in good faith
- Do not exploit vulnerabilities beyond PoC

---

## Responsible Disclosure Policy

### Our Commitment

- **Acknowledgment:** Within 48 hours
- **Transparency:** Regular updates on progress
- **Recognition:** Public acknowledgment (if desired)
- **Timeline:** Fix within 90 days (or mutual extension)

### Your Responsibility

- **Confidentiality:** Do not publicly disclose before our fix
- **Coordination:** Work with us on disclosure timeline
- **Patience:** Allow reasonable time for remediation

### Disclosure Timeline

1. **Day 0:** Vulnerability reported
2. **Day 0-5:** Triage and validation
3. **Day 5-90:** Fix development
4. **Day 90:** Coordinated public disclosure
5. **Day 97:** Bounty payment

---

## Examples of Valid Submissions

### ✅ Good Submission

**Title:** Authentication Bypass in Mining Pool API

**Description:**  
The mining pool API endpoint `/api/pool/stats` does not properly validate JWT tokens, allowing an attacker to access statistics for any pool by manipulating the `pool_id` parameter.

**Impact:**  
- Unauthorized access to pool statistics
- Exposure of miner addresses and earnings
- Potential manipulation of pool rewards

**PoC:**
```bash
curl -X GET https://miningpool.carphatian.ro/api/pool/stats \
  -H "Authorization: Bearer <expired_token>" \
  -d "pool_id=1234"
```

**Remediation:**  
Implement proper token validation and verify pool ownership before returning data.

---

### ❌ Bad Submission

**Title:** SQL Injection

**Description:**  
There's SQL injection in your website.

**PoC:**  
I tested with sqlmap and it found vulnerabilities.

**Issues:**
- Too vague
- No specific endpoint
- No manual verification
- Automated scanner output

---

## Contact

**Security Team:**
- Email: security@antimony.network
- PGP Key: Available at https://antimony.network/pgp
- HackerOne: https://hackerone.com/antimony
- Discord: #security channel

**Emergency Contact:**
For critical vulnerabilities requiring immediate attention:
- Email: critical-security@antimony.network
- Response Time: < 4 hours

---

## Hall of Fame

Top contributors to ATMN security:

🏆 **2025 Q4**
1. _Awaiting first submissions_
2. _Join our hall of fame!_
3. _Help secure ATMN blockchain_

---

## Legal

This bug bounty program is administered by Antimony Foundation. By participating, you agree to:

- These terms and conditions
- Our Privacy Policy
- Responsible disclosure practices
- Applicable laws and regulations

Antimony Foundation reserves the right to:
- Modify or terminate this program
- Change reward amounts
- Disqualify submissions
- Request additional information

---

## FAQs

**Q: Can I test on mainnet?**  
A: Only with explicit written permission. Use testnet first.

**Q: How do I get test ATMN?**  
A: Use our faucet: https://faucet.antimony.network

**Q: What if someone else reports the same bug?**  
A: First valid, detailed submission wins.

**Q: Can I work with a team?**  
A: Yes, reward will be split as you specify.

**Q: Do you accept anonymous submissions?**  
A: Yes, but you must provide valid ATMN address for payment.

**Q: What languages do you accept?**  
A: English preferred. We accept Romanian and Spanish.

**Q: How is bounty amount determined?**  
A: Based on CVSS score, impact, quality, and our assessment.

---

**Last Updated:** December 6, 2025  
**Version:** 1.0  
**Program Status:** 🟢 ACTIVE

For questions: bounty@antimony.network
