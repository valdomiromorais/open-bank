# μBank — Seu dinheiro é código. Sua conta é uma chave privada.

> *Um cofre digital distribuído. Construído em Rust. Blindado por design.*

**μBank** não é uma **fintech colorida**. É um **sistema financeiro de grade militar**, onde cada transação é verificada,
cada moeda é tipada, e cada erro de domínio é capturado em compile-time — não em produção.
Nada de curvas amigáveis. Arquitetura de **hardware security module**, estética [red team](docs/MUB_GLOSSARY.md#red-team), precisão criptográfica.



Três interessados coexistem no μBank:

| Operador          | Missão                                                                                                                                                                |
|-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Interessado 1** | Domínio de arquitetura financeira segura em **Rust**, com comparações deliberadas em **C** e **C++** para dissecar trade-offs de segurança, desempenho e complexidade |
| **Interessado 2** | **Modo LEARNING** — jovens exploram matemática financeira em ambiente sandbox com moeda **MUB**, sem risco real                                                       |
| **Interessado 3** | Roteiro regulatório (BACEN, LGPD, Open Finance) para um produto real operando com moedas fiduciárias (USD, EUR, BRL)                                                  |

---

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

## Estado atual do cofre

- ✅ Workspace Rust — 9 crates, resolver 3, compile em paralelo
- ✅ `mu_core` — `Currency` (MUB, USD, EUR, BRL) + `Money` (`rust_decimal`, Add/Sub com `Result`, 13 testes)
- 🟡 Demais crates — stubs, aguardando ativação
- 📚 Documentação viva em `docs/` (roadmap, context memory, branding, briefings)

---

## Como usar

```bash
# Compilar tudo (sem aviso, sem erro)
cargo build

# Verificar a blindagem do core
cargo test -p mu_core

# Executar a interface de comando (MVP textual)
cargo run -p mu_cli
```

## Licença

MIT — porque segurança não precisa de código fechado.
