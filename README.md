# μBank — Um Openbank

> *Um banco de todos e para todos.*

**μBank** é um projeto educacional e de engenharia de software focado em sistemas financeiros. Construído primariamente em **Rust**, com comparações deliberadas em **C** e **C++** para explorar trade-offs de segurança, desempenho e complexidade.

## Missão

O projeto atende três interessados simultaneamente:

| Interessado | Foco |
|---|---|
| **1 — Eu** | Aprendizado de arquitetura financeira, engenharia de software seguro, e domínio de Rust + C/C++ |
| **2 — Jovens estudantes** | Ambiente simulado **LEARNING** com moeda **MUB** para explorar matemática financeira e operações bancárias |
| **3 — Mercado futuro** | Roteiro regulatório (BACEN, LGPD, Open Finance) para um produto real operando com moedas fiduciárias (USD, EUR, BRL) |

## Arquitetura

```
μBank
├── mu_core/        Tipos de domínio: Currency, Money, Account, Ledger
├── mu_cli/         Interface de linha de comando (MVP textual)
├── mu_api/         API HTTP (Axum — planejado)
├── mu_db/          Persistência e Event Store (SQLite — planejado)
├── mu_protocol/    Tipos compartilhados para comunicação entre serviços
├── mu_sim/         Motor do modo LEARNING (moeda MUB)
├── mu_security/    Hash, KYC, criptografia
├── mu_audit/       Log estruturado e trilha de auditoria
└── mu_bench/       Benchmarks e harness de comparação C/C++
```

## Estado atual

- ✅ Workspace Rust configurado com 9 crates
- ✅ `mu_core` — `Currency` (MUB, USD, EUR, BRL) + `Money` (Decimal preciso, Add/Sub seguros)
- 🟡 Demais crates — stubs aguardando implementação
- 📚 Documentação em `docs/` (roadmap, context memory, branding)

## Como usar

```bash
# Compilar tudo
cargo build

# Rodar testes do core
cargo test -p mu_core

# Executar o CLI (MVP inicial)
cargo run -p mu_cli
```

## Licença

MIT
