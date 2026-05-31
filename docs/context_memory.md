# Context Memory — Mu-Bank (μBank)

> Última atualização: 2026-05-30

---

## 1. Projeto

- **Nome:** μBank (Mu-Bank), apelido `open-bank`
- **Missão:** Banco digital mais seguro da Terra — um cofre blindado por design — três interessados coexistem:
  - **Interessado 1 (eu):** aprendizado de engenharia/arquitetura financeira em Rust (primário), C e C++ (comparação)
  - **Interessado 2 (jovens):** simulação bancária modo LEARNING com moeda MUB para aprender matemática financeira
  - **Interessado 3 (futuro):** produto comercial real (USD, EUR, BRL) com _Compliance_ (Conformidade) legal (BACEN, LGPD, Open Finance)
- **Prazo:** 24 meses (8 trimestres), vide `docs/roadmap.md`
- **Prompt original:** `docs/initial_prompt.md`

## 2. Estado atual (commit `8aec2a3`)

### Workspace Rust (`Cargo.toml` raiz como `[workspace]`)

```
crates/
├── mu_core/        # ✅ ATIVO — Currency, Money, Account, Customer, Transaction, Ledger
├── mu_cli/         # ✅ DEMO FUNCIONAL — cria clientes, contas, deposita, saca, transfere
├── mu_api/         # 🟡 stub
├── mu_db/          # 🟡 stub
├── mu_protocol/    # 🟡 stub
├── mu_sim/         # 🟡 stub
├── mu_security/    # 🟡 stub
├── mu_audit/       # 🟡 stub
└── mu_bench/       # 🟡 stub
```

### Crate `mu_core` — implementado

- **`Currency`** (`currency.rs`): enum `MUB | USD | EUR | BRL | CNY | GBP`
  - ISO 4217 numeric codes (MUB = 999 privado)
  - `code()`, `symbol()`, `name()`, `decimals()`
  - `TryFrom<&str>`, `Display`
- **`Money`** (`money.rs`): `{ amount: Decimal, currency: Currency }`
  - `rust_decimal::Decimal` (i128, 28 dígitos, sem ponto flutuante)
  - `Add`/`Sub` devolvem `Result<Money, MoneyError>`
  - Protege contra: mistura de moedas → `CurrencyMismatch`, overflow → `Overflow`
  - Display com casas decimais fixas conforme `Currency::decimals()`
- **`Customer`** (`customer.rs`): `CustomerId(Uuid)`, `Customer { id, name }`
- **`Account`** (`account.rs`): `AccountId(Uuid)`, `AccountStatus { Active, Frozen, Closed, PendingVerification }`, `Account { id, holder, status, currency }`
- **`Transaction`** (`transaction.rs`): `TransactionId(Uuid)`, `TransactionKind { Deposit, Withdraw, Transfer, Reversal { original_tx } }`, `Transaction { id, account_id, kind, amount, timestamp, description }`
- **`Ledger`** (`ledger.rs`): engine central in-memory com:
  - Customer CRUD, Account CRUD com activation
  - `balance()` computado por fold no histórico (Event Sourcing)
  - `deposit()`, `withdraw()`, `transfer()`, `reversal()` com validações
  - Proteções: conta inativa, moeda errada, saldo insuficiente, auto-transferência, reversão dupla bloqueada
- **37 testes unitários passando** (20 originais + 17 novos estorno)

### Docs — alinhamento de identidade (2026-05-26)

- `README.md` reescrito com a vibe red team / cofre digital / cibersegurança agressiva
- `docs/MANIFEST.md` criado — manifesto dopaminérgico diário do desenvolvedor
- `docs/domain_design_document.md` absorvido — define 8 bounded contexts, monólito modular, ledger imutável, eventos de domínio, partidas dobradas
- Vibe consolidada: estética hardware security module, preto + verde terminal + dourado frio, "seu dinheiro é código, sua conta é uma chave privada"

### ✅ Concluído (2026-05-30)

- **`TransactionKind::Reversal { original_tx }`** — novo variante do enum para estorno de transações
- **`Ledger.reversal()`** — engine de estorno com validação: só permite estornar Deposit/Withdraw/Transfer, bloqueia dupla reversão, verifica conta/moeda/saldo, gera descrição automática
- **37 testes unitários** — 17 novos testes para estorno (sucesso, falha por tipo inválido, dupla reversão, conta errada, moeda errada, saldo insuficiente)
- **Refatoração de construtores** — `Account::new()`, `Customer::new()`, `Transaction::new()` auto-geram UUIDs; `with_id()` criado para restauração de persistência
- **Temas JetBrains** — `muBank_jetbrains_theme.icls` (pure black μBank) e `muBank_nord_theme.icls` (μBank × Nord)
- **Arquitetura visual** — prompts Mermaid renderizados em `docs/images/` (visão geral, ledger imutável, ledger × conta corrente)
- **`Cargo.lock` e `.gitignore`** atualizados com `target/` e diretórios IDE
- **Remote sincronizado** — `git push` realizado, working tree clean, 0 commits behind
- **`docs/FINTECHS.md`** — relatório com análise de 7 fintechs brasileiras (Nubank, PicPay, PagBank, Mercado Pago, Stone, Creditas, Conta Azul)

### Análise Fintechs — Aproveitamento para o μBank

| Insight | Origem | Ação |
|---------|--------|------|
| **Monólito modular validado** | Todas as 7 fintechs | Nubank e Mercado Pago só migraram para microsserviços após escala comprovada. Nossa escolha de começar monolítico é o padrão de sucesso brasileiro. |
| **Event Sourcing como backbone** | Nubank (Datomic/Clojure) | Já implementamos (`balance()` é fold de eventos). Reforçar como nosso "Datomic". |
| **Boleto + Parcelamento (BNPL)** | Mercado Pago, PagBank | Duas operações bancárias essenciais que faltam no `TransactionKind`. Adicionar `Boleto` e `Installment`. |
| **Cashback/Rewards para LEARNING** | PicPay, Stone (gamificação) | `TransactionKind::Cashback` para engajar Interessado 2 no modo LEARNING com missões e recompensas MUB. |
| **API no modelo BaaS** | Conta Azul | REST API do `mu_api` deve ser desenhada para qualquer ERP conectar — chave de API, versionamento `/v1/`, documentação OpenAPI. |
| **PCI-DSS como meta** | PagBank | Adicionar estudo dos 12 requisitos PCI-DSS no Trimestre 4 como meta de compliance de segurança. |

## 3. Decisões arquiteturais

| Decisão | Justificativa |
|---------|---------------|
| `[workspace]` com resolver 3 | Múltiplos crates independentes compilando em paralelo |
| `amount` como `Decimal` (não `i64` ou `f64`) | Precisão exata financeira, sem arredondamento IEEE 754 |
| `Currency` separado de `Money` | `Money = amount + currency`; operações cross-currency são barradas em compile-time (runtime) |
| `Add`/`Sub` com `Output = Result` | Erros de domínio (moeda errada, overflow) são tratados, não panics |
| `#[derive(Copy)]` em `Money` e `Currency` | Tipos pequenos e bitwise-copyable, sem necessidade de `Clone` explícito |
| `resolver = "3"` | Feature unification mais previsível para workspaces com muitas deps |
| **Monólito modular** (não microservices no início) | Baseado na análise Nubank × Openbank em `docs/domain_design_document.md` — começar simples, evoluir por extração de módulos |
| **DDD + Bounded Contexts** para modelagem de domínio | 8 contextos mapeados: Identity, Accounts, Ledger, Payments, Cards, Credit, Education, Compliance |
| **Ledger imutável com partidas dobradas** | Toda movimentação financeira exige lançamento contábil correspondente (double-entry) |
| **Eventos de domínio como cidadãos de primeira classe** | `CustomerRegistered`, `MoneyDeposited`, `TransferCompleted` etc. — base para audit trail e event sourcing futuro |
| **Formatação de valores separada do domínio** | `Money::Display` NÃO terá separadores de milhar/locale — formatação é preocupação de apresentação. Uma função separada `format_money(money, locale)` em `mu_cli` ou `mu_api` será criada no futuro. |

## 4. Convenções de código

- **Idioma:** código e comentários em inglês
- **Comentários em português:** marcados com `#[ptbr]` no início, explicando o propósito do trecho
  - **⚠️ Importante:** o Interessado 1 pretende remover **todos** os comentários `#[ptbr]` futuramente, quando julgar que o aprendizado foi satisfatório. Isso pode ser feito com uma regex simples do tipo `//\s*#\[ptbr\].*\n` em todo o código-base.
- **Estilo:** sem comentários supérfluos; usar tipos e nomes expressivos

## 5. Identidade visual

- **Fonte principal:** JetBrains Mono (monoespaçada técnica) — usada em logo, numeração de cartão, interface e código
- **Paleta:** preto absoluto (`#000000`), verde terminal (`#00FF41`), dourado frio (`#D4AF37`), vermelho sangue digital (`#FF0040`)
- **Estética:** hardware security module, red team, sem curvas amigáveis

## 6. Alinhamento roadmap × prompt

- Roadmap `docs/roadmap.md` abrange os 24 meses em 8 trimestres
- Alinhamento quase total com o prompt (ver `docs/roadmap.md` seção de análise)
- **Único gap:** a convenção `#[ptbr]` não está explícita no roadmap — está decidida neste contexto

## 7. Progresso — Trimestre 1

### ✅ Concluído (2026-05-26)

- `docs/MUB_GLOSSARY.md` — glossário com 40+ termos
- `.github/PULL_REQUEST_TEMPLATE.md` — template de PR (GitHub Flow)
- `docs/domain_design_document.md` — referência arquitetural absorvida
- **`mu_core`**: `Account`, `Customer`, `Transaction`, `Ledger` modelados (29 testes)
- **`mu_cli`**: demo funcional — cria cliente, conta, deposita, saca, transfere, extrato
- **`Currency`**: expandido com `CNY`, `GBP` e `JPY` (7 moedas — MUB, USD, EUR, BRL, CNY, GBP, JPY)
- `README.md`, `docs/context_memory.md`, `docs/roadmap.md` — alinhados e sincronizados

### ✅ Concluído (2026-05-27)

- `docs/MUB_GLOSSARY.md` — anotações `#[ptbr]` em todos os headings, nova seção `UUID` detalhada, KYC expandido
- `crates/mu_core/src/customer.rs` — `#[ptbr]` sobre Newtype pattern e UUID (128 bits, versões, formato 8-4-4-4-12)
- `Currency::JPY` adicionado ao enum com símbolo `JP¥` (ISO 4217: 392)
- **Observação:** glossário será traduzido para inglês futuramente (Interessado 1)

### 🔜 Próximos passos (próximas semanas)

- Implementar `TransactionKind::Boleto` e `TransactionKind::Installment` (BNPL)
- Implementar `TransactionKind::Cashback` para gamificação LEARNING
- Persistência com SQLite/`sqlx`
- API HTTP com Axum no modelo BaaS

## 8. Observações gerais

- Sempre que possível, prefira bibliotecas bem estabelecidas (ex: `rust_decimal` em vez de implementar do zero)
- O remote é `origin → https://github.com/valdomiromorais/open-bank.git`
- Commits são em inglês, descritivos, com escopo do crate afetado (`feat(mu_core): ...`, `docs: ...`, `chore: ...`)
- Fluxo de trabalho: GitHub Flow simplificado — branches curtas `feat/`, PRs com template próprio em `.github/PULL_REQUEST_TEMPLATE.md`

## 9. Arquivos vivos — atualização contínua

A cada conclusão de tópico, os seguintes arquivos devem ser atualizados para refletir o estado real do projeto:

| Arquivo | Propósito |
|---------|-----------|
| `docs/context_memory.md` | Estado atual, decisões, convenções, próximos passos |
| `docs/roadmap.md` | Progresso dos checklists, checkboxes marcados, ajustes de cronograma |
| `README.md` | Visão geral, status dos crates, instruções de uso |
| `docs/MANIFEST.md` | Manifesto do desenvolvedor — frase dopaminérgica diária e marcos |

Isso garante que a documentação nunca fique dessincronizada do código e que o Interessado 1 (e qualquer IA auxiliar) sempre tenha contexto fiel ao projeto real.
