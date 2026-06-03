# Prompt — Diagrama de Arquitetura do μBank

> Use com Midjourney, DALL-E, Stable Diffusion ou similar.

---

High-level architecture diagram of the μBank (Mu-Bank) digital banking system. Black background (#000000). Hardware security module aesthetic, red team / cybersecurity vibe. Monospace font (JetBrains Mono), sharp angles, no curves, no gradients, high contrast.

**Layout:** Three horizontal layers connected by solid gold (#D4AF37) arrows flowing downward and sideways.

---

## Layer 1: Interfaces (top, dark gray boxes #222222)

- **mu_cli** — Terminal/CLI interface. Small box. Text inside: "mu_cli — demo terminal client. Deposit, Withdraw, Transfer, Statement."
- **mu_api** — REST API (future). Dashed outline, 50% opacity. Text: "mu_api — Axum REST API. BaaS model."

## Layer 2: Core Engine (center, large, terminal green #00FF41 border)

Central box **mu_core** with internal components:

1. **Ledger** (largest box, white text on #1A1A1A) — "Single Source of Truth. balance() = fold(transactions). Event Sourcing."
   - Internal annotation: "Vec<Transaction> — never DELETE, never UPDATE. Corrections = Reversal."

2. **Transaction** (box, blood red #FF0040) — "TransactionKind: Deposit | Withdraw | Transfer | Reversal{original_tx} | BankslipPayment{code}"

3. **Account** (box, terminal green #00FF41) — "AccountId(Uuid), holder: CustomerId, status: Active/Frozen/Closed/PendingVerification"

4. **Customer** (box, military cyan #00E5FF) — "CustomerId(Uuid), name: String"

5. **Money** (box, cold gold #D4AF37) — "amount: Decimal (rust_decimal, i128), currency: Currency. Add/Sub → Result. No f64."

6. **Currency** (box, terminal green #00FF41) — "enum: MUB, USD, EUR, BRL, CNY, GBP, JPY. ISO 4217 codes."

7. **BankSlip** (box, warning amber #FF8C00) — "Instrument separated from Transaction. lifecycle: Pending → Registered → Paid | Cancelled | Expired."

## Layer 3: Storage & Security (bottom, dark gray boxes #222222)

- **mu_db** (dashed outline, 50% opacity) — "SQLite/sqlx (future)"
- **mu_audit** (dashed outline, 50% opacity) — "Audit trail (future)"
- **mu_security** (dashed outline, 50% opacity) — "Argon2, KYC, LGPD (future)"
- **mu_sim** (dashed outline, 50% opacity) — "LEARNING mode gamification (future)"

**Callout box at top-right corner (digital blood red #FF0040, small text):**
"44 tests passing | Cantares 2:15 — 'Apanhai-nos as raposinhas'"

**Watermark at bottom-right (#D4AF37, 30% opacity):**
"Sua conta é uma chave privada"

**Arrows (#D4AF37):**
- mu_cli → Ledger (via library calls)
- mu_api → Ledger (future, dashed arrow)
- Ledger ↔ Transaction (writes and reads)
- Transaction → Account (references via AccountId)
- Account → Customer (references via CustomerId)
- Transaction → Money (amount field)
- Money → Currency (currency field)
- BankSlip → Ledger (stored in Vec<BankSlip>)
- BankSlip → Transaction (when paid, creates BankslipPayment)
- Ledger ↓ mu_db (future persistence, dashed)
- Ledger ↓ mu_audit (future, dashed)

**Style:** Technical manual / hardware security module. Clean lines, high contrast, monospace. Industrial cybersecurity feel — like a bank vault blueprint drawn by a red team engineer.
