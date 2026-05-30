# Prompt para Diagrama de Arquitetura do μBank

## Paleta de cores oficial μBank

| Cor                     | Hex       | Uso                               |
|-------------------------|-----------|-----------------------------------|
| Preto absoluto          | `#000000` | Fundo                             |
| Cinza-chumbo            | `#111116` | Superfícies secundárias           |
| Verde terminal          | `#00FF41` | Elementos principais, glow, texto |
| Dourado frio            | `#D4AF37` | Bordas, contornos, módulos        |
| Vermelho sangue digital | `#FF0040` | Glitches, alertas                 |
| Ciano militar           | `#00E5FF` | Fluxos de dados, setas            |

## Prompt principal (inglês, otimizado para DALL·E 3 / Midjourney / Stable Diffusion)

```
A technical architecture diagram of "μBank" as a modular monolith written in Rust, cybersecurity HUD style. Color palette: absolute black (#000000) background with lead-gray (#111116) secondary surfaces, terminal green (#00FF41) for primary elements and neon glow, cold gold (#D4AF37) for module borders, military cyan (#00E5FF) for data flow arrows, and digital blood red (#FF0040) for subtle glitch accents.

The center is a glowing angular vault or cryptographic processor icon labeled "LEDGER — Encrypted Event Store" in terminal green (#00FF41) with neon bloom, showing a chain of blocks representing cryptographic hash chaining (each event hash-linked to the previous). A Rust "crab" silhouette (Ferris) embedded as a watermark in the center module.

Surrounding the center in an octagonal ring layout, 8 modules as dark angular hexagons with thin cold gold (#D4AF37) borders and cyan (#00E5FF) encrypted data flow arrows between them:

1. "IDENTITY & ONBOARDING" — person + document + argon2 key icon, top-left
2. "ACCOUNTS" — wallet with padlock, top
3. "LEDGER" — (center, cryptographic chain)
4. "PAYMENTS" — arrow with encryption shield, top-right
5. "CARDS" — credit card with EMV chip octagonal, right
6. "CREDIT" — coin with interest rate graph, bottom-right
7. "EDUCATION LAYER" — graduation cap + MUB currency symbol (μ), bottom-left
8. "COMPLIANCE & AUDIT" — shield + checklist + KYC badge, left

Encrypted data flow lines in military cyan (#00E5FF) with directional arrows connecting all modules to the central LEDGER, showing that every financial operation is cryptographically signed and recorded in the immutable ledger. Small lock icons and hexadecimal fragments ("0xAES256", "0xED25519", "0xBLAKE3") floating in the background.

The word "μBank" in JetBrains Mono monospaced font at the top in terminal green (#00FF41) with a subtle glow. Security badges on the sides: "AES-256-GCM", "Argon2id", "ED25519", "SHA-3".

Bottom text in smaller monospaced font: "RUST — MODULAR MONOLITH — 8 BOUNDED CONTEXTS — CRYPTOGRAPHIC LEDGER — BLINDADO POR DESIGN"

Style: cyber-aggressive fintech, hardware security module aesthetic, red team, technical blueprint meets hacker command center, 4K, sharp focus, no curves — all hard angles and angular geometry.
```

## Variação minimalista (para slide/docs)

```
Same "μBank" architecture diagram, flat design, simplified: 8 rectangular modules in circular layout around center "LEDGER (Cryptographic Event Store)" box. Color palette: black (#000000) background, terminal green (#00FF41) for module borders and text, white arrows for data flow. A small Rust Ferris icon next to the title. No glitch, no gold, no cyan. Clean technical documentation style.
```
