# Context Memory — Mu-Bank (μBank)

> Última atualização: 2026-05-26

---

## 1. Projeto

- **Nome:** μBank (Mu-Bank), apelido `open-bank`
- **Missão:** Banco digital mais seguro da Terra — um cofre blindado por design — três interessados coexistem:
  - **Interessado 1 (eu):** aprendizado de engenharia/arquitetura financeira em Rust (primário), C e C++ (comparação)
  - **Interessado 2 (jovens):** simulação bancária modo LEARNING com moeda MUB para aprender matemática financeira
  - **Interessado 3 (futuro):** produto comercial real (USD, EUR, BRL) com合规 legal (BACEN, LGPD, Open Finance)
- **Prazo:** 24 meses (8 trimestres), vide `docs/roadmap.md`
- **Prompt original:** `docs/initial_prompt.md`

## 2. Estado atual (commit `3f8fb07` + alterações não commitadas)

### Workspace Rust (`Cargo.toml` raiz como `[workspace]`)

```
crates/
├── mu_core/        # ✅ ATIVO — Currency + Money (rust_decimal)
├── mu_cli/         # 🟡 stub — main.rs só imprime placeholder
├── mu_api/         # 🟡 stub
├── mu_db/          # 🟡 stub
├── mu_protocol/    # 🟡 stub
├── mu_sim/         # 🟡 stub
├── mu_security/    # 🟡 stub
├── mu_audit/       # 🟡 stub
└── mu_bench/       # 🟡 stub
```

### Crate `mu_core` — implementado

- **`Currency`** (`currency.rs`): enum `MUB | USD | EUR | BRL`
  - ISO 4217 numeric codes (MUB = 999 privado)
  - `code()`, `symbol()`, `name()`, `decimals()`
  - `TryFrom<&str>`, `Display`
- **`Money`** (`money.rs`): `{ amount: Decimal, currency: Currency }`
  - `rust_decimal::Decimal` (i128, 28 dígitos, sem ponto flutuante)
  - `Add`/`Sub` devolvem `Result<Money, MoneyError>`
  - Protege contra: mistura de moedas → `CurrencyMismatch`, overflow → `Overflow`
  - Display com casas decimais fixas conforme `Currency::decimals()`
  - 13 testes unitários passando

### Docs — alinhamento de identidade (2026-05-26)

- `README.md` reescrito com a vibe red team / cofre digital / cibersegurança agressiva
- `docs/MANIFEST.md` criado — manifesto dopaminérgico diário do desenvolvedor
- `docs/domain_design_document.md` absorvido — define 8 bounded contexts, monólito modular, ledger imutável, eventos de domínio, partidas dobradas
- Vibe consolidada: estética hardware security module, preto + verde terminal + dourado frio, "seu dinheiro é código, sua conta é uma chave privada"

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

## 4. Convenções de código

- **Idioma:** código e comentários em inglês
- **Comentários em português:** marcados com `#[ptbr]` no início, explicando o propósito do trecho
  - **⚠️ Importante:** o Interessado 1 pretende remover **todos** os comentários `#[ptbr]` futuramente, quando julgar que o aprendizado foi satisfatório. Isso pode ser feito com uma regex simples do tipo `//\s*#\[ptbr\].*\n` em todo o código-base.
- **Estilo:** sem comentários supérfluos; usar tipos e nomes expressivos

## 5. Alinhamento roadmap × prompt

- Roadmap `docs/roadmap.md` abrange os 24 meses em 8 trimestres
- Alinhamento quase total com o prompt (ver `docs/roadmap.md` seção de análise)
- **Único gap:** a convenção `#[ptbr]` não está explícita no roadmap — está decidida neste contexto

## 6. Progresso — Trimestre 1

### ✅ Concluído (2026-05-26)

- `docs/MUB_GLOSSARY.md` criado com 40+ termos da linguagem ubíqua, abrangendo 8 bounded contexts
- Roadmap checkbox marcado (`[x]`)

### 🔜 Próximos passos

Conforme roadmap, semanas 1-2 continuam com:
- Operações bancárias essenciais (Débito, Crédito, Transferência)
- Modelagem de `Account`, `Customer`, `Ledger`, `Transaction` em `mu_core`

## 7. Observações gerais

- Sempre que possível, prefira bibliotecas bem estabelecidas (ex: `rust_decimal` em vez de implementar do zero)
- O remote é `origin → https://github.com/valdomiromorais/open-bank.git`
- Commits são em inglês, descritivos, com escopo do crate afetado (`feat(mu_core): ...`)

## 8. Arquivos vivos — atualização contínua

A cada conclusão de tópico, os seguintes arquivos devem ser atualizados para refletir o estado real do projeto:

| Arquivo | Propósito |
|---------|-----------|
| `docs/context_memory.md` | Estado atual, decisões, convenções, próximos passos |
| `docs/roadmap.md` | Progresso dos checklists, checkboxes marcados, ajustes de cronograma |
| `README.md` | Visão geral, status dos crates, instruções de uso |
| `docs/MANIFEST.md` | Manifesto do desenvolvedor — frase dopaminérgica diária e marcos |

Isso garante que a documentação nunca fique dessincronizada do código e que o Interessado 1 (e qualquer IA auxiliar) sempre tenha contexto fiel ao projeto real.
