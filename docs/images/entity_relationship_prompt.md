# Prompt — Diagrama de Relacionamentos das Entidades μBank

> Use com Midjourney, DALL-E, Stable Diffusion ou similar.

---

Entity-relationship diagram of the μBank core domain. Black background (#000000). Technical diagram style, hardware security module aesthetic. Seven entity boxes connected by directed arrows.

**Entity boxes:**
1. **Currency** (enum) — 7 variants: MUB, USD, EUR, BRL, CNY, GBP, JPY. Box color: terminal green #00FF41.
2. **Money** (struct) — fields: amount (Decimal), currency (Currency). Box color: cold gold #D4AF37.
3. **CustomerId** (newtype Uuid) — small box, light gray.
4. **Customer** (struct) — fields: id (CustomerId), name (String). Box color: military cyan #00E5FF.
5. **AccountId** (newtype Uuid) — small box, light gray.
6. **Account** (struct) — fields: id (AccountId), holder (CustomerId), status (AccountStatus {Active, Frozen, Closed, PendingVerification}), currency (Currency). Box color: terminal green #00FF41.
7. **TransactionId** (newtype Uuid) — small box, light gray.
8. **Transaction** (struct) — fields: id (TransactionId), account_id (AccountId), kind (TransactionKind {Deposit, Withdraw, Transfer, Reversal}), amount (Money), timestamp, description. Box color: blood red #FF0040.
9. **Ledger** (struct) — fields: customers (HashMap), accounts (HashMap), transactions (Vec). Box color: white text, dark gray border. Central/hub position.

**Relationships (gold arrows #D4AF37):**
- Currency ──→ Money (is a field of)
- Customer ──→ Account (via CustomerId as holder)
- Money ──→ Transaction (is the amount field)
- Account ──→ Transaction (via AccountId as account_id)
- Transaction ──→ Ledger (stored in Vec<Transaction>)
- Account → Ledger (stored in HashMap<AccountId, Account>)
- Customer → Ledger (stored in HashMap<CustomerId, Customer>)
- Ledger → Ledger (loop arrow labeled "balance() = fold(transactions)" — Event Sourcing, balance never stored)

**Style:** Clean lines, monospace font (JetBrains Mono), no curves, no gradients, sharp angles. Red team / cybersecurity aesthetic. "Sua conta é uma chave privada" watermark at bottom right in small golden text. High contrast, minimal, technical manual look.
