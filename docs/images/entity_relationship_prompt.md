# Prompt — Diagrama de Relacionamentos das Entidades μBank

> Use com Midjourney, DALL-E, Stable Diffusion ou similar.

---

Entity-relationship diagram of the μBank core domain. Black background (#000000). Technical diagram style, hardware security module aesthetic. Entity boxes connected by directed gold arrows.

**Entity boxes:**

1. **Currency** (enum) — 7 variants: MUB, USD, EUR, BRL, CNY, GBP, JPY. Box color: terminal green #00FF41.

2. **Money** (struct) — fields: amount (Decimal), currency (Currency). Box color: cold gold #D4AF37.

3. **CustomerId** (newtype Uuid) — small auxiliary box, light gray.
   **Customer** (struct) — fields: id (CustomerId), name (String). Box color: military cyan #00E5FF.

4. **AccountId** (newtype Uuid) — small auxiliary box, light gray.
   **Account** (struct) — fields: id (AccountId), holder (CustomerId), status (AccountStatus {Active, Frozen, Closed, PendingVerification}), currency (Currency). Box color: terminal green #00FF41.

5. **TransactionId** (newtype Uuid) — small auxiliary box, light gray.
   **Transaction** (struct) — fields: id (TransactionId), account_id (AccountId), kind (TransactionKind {Deposit, Withdraw, Transfer, Reversal {original_tx}, BankslipPayment {code}}), amount (Money), timestamp (DateTime<Utc>), description (String). Box color: blood red #FF0040.
   Note on box: "Reversal links back to original Transaction via original_tx: TransactionId".

6. **BankSlip** (struct) — fields: id (Uuid), account_id (AccountId), amount (Money), code (String), status (SlipStatus {Pending, Registered, Paid, Cancelled, Expired}), due_date (NaiveDate), issued_at (DateTime<Utc>), paid_at (Option<DateTime<Utc>>). Box color: warning amber #FF8C00.
   **SlipStatus** (enum) — small auxiliary box near BankSlip, showing the lifecycle: Pending → Registered → Paid | Cancelled | Expired.

7. **Ledger** (struct) — fields: customers (HashMap<CustomerId, Customer>), accounts (HashMap<AccountId, Account>), transactions (Vec<Transaction>), bankslips (Vec<BankSlip>). Box color: white text, dark gray border (#333333). Central/hub position, larger than others.
   Note on box: "Single source of truth — balance() = fold(transactions)"

**LedgerError** (enum) — not a box, shown as a floating callout list near Ledger:
  - AccountNotFound, CustomerNotFound, AccountNotActive
  - InsufficientBalance, CurrencyMismatch, MoneyError
  - TransferToSameAccount, TransactionNotFound
  - OriginalAlreadyReversed, BankslipNotFound, BankslipAlreadyPaid

**Relationships (gold arrows #D4AF37):**

- Currency ──→ Money (is field of)
- Money ──→ BankSlip (is the amount field)
- Money ──→ Transaction (is the amount field)
- CustomerId ──→ Customer (id field)
- CustomerId ──→ Account (via holder field)
- AccountId ──→ Account (id field)
- AccountId ──→ Transaction (via account_id field)
- AccountId ──→ BankSlip (via account_id field)
- Transaction ──→ Ledger (stored in Vec<Transaction>)
- BankSlip ──→ Ledger (stored in Vec<BankSlip>)
- Account ──→ Ledger (stored in HashMap<AccountId, Account>)
- Customer ──→ Ledger (stored in HashMap<CustomerId, Customer>)
- Transaction ──→ Transaction (loop arrow labeled "Reversal.original_tx references another Transaction")
- Ledger ──→ Ledger (loop arrow labeled "balance() = fold(transactions)" — Event Sourcing)

**Style:** Clean lines, monospace font (JetBrains Mono), no curves, no gradients, sharp angles, high contrast. Red team / cybersecurity aesthetic. "Sua conta é uma chave privada" watermark at bottom right corner in small golden (#D4AF37, 50% opacity) text. Minimal, technical manual / hardware security module look.
