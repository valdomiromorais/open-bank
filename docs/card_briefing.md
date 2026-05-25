## μBank Black Card — Cartão de Crédito Físico

### Conceito Geral

Um cartão que parece um dispositivo criptográfico. Não é um cartão bancário comum: é um **hardware token de acesso** a um sistema financeiro blindado. A mensagem é: *"Este não é um cartão. É uma chave privada física."*

---

### Especificações Técnicas do Cartão

| Elemento       | Especificação                                                              |
|:---------------|:---------------------------------------------------------------------------|
| **Material**   | Metal escovado preto (black brushed metal), pesado ao toque                |
| **Acabamento** | Fosco total, sem brilho, com toque aveludado (soft-touch)                  |
| **Espessura**  | Ligeiramente mais grosso que um cartão padrão (0.9mm), sensação de solidez |
| **Bordas**     | Chanfradas em ângulo, cor de cobre oxidado ou dourado frio                 |

---

### Face Frontal (Frente)

```
┌─────────────────────────────────────────────┐
│                                             │
│   ███ ███ ███ ███   (chip criptográfico)   │
│   ███ ███ ███ ███     dourado frio          │
│                                             │
│          ╔═══════════════╗                  │
│          ║      μ        ║  ← runa central  │
│          ║   (glifo)     ║     com chave    │
│          ╚═══════════════╝     criptográfica│
│                                             │
│     5333  0917  4428  7761                  │
│     (numeração em verde terminal)           │
│                                             │
│     VALID THRU  12/28                       │
│                                             │
│     ΛΙΛ SΙLVA                               │
│     (nome em monoespaçado)                  │
│                                             │
└─────────────────────────────────────────────┘
```

**Detalhamento da frente:**

- **Canto superior esquerdo:** logotipo "μBank" em verde terminal (#00FF41) com glow suave, como se estivesse impresso com tinta luminescente.
- **Centro-direita:** chip EMV com design personalizado — em vez do padrão quadrado comum, um chip octogonal com micro-texto gravado: "TRUST NO ONE" em letras quase invisíveis.
- **Centro:** o símbolo μ estilizado (runa + chave criptográfica + escudo angular) em **baixo-relevo**, preenchido com tinta metálica que muda de cor conforme o ângulo: verde terminal → dourado frio.
- **Numeração do cartão:** fonte monoespaçada (JetBrains Mono), cor **verde terminal (#00FF41)**, espaçamento generoso entre grupos de 4 dígitos.
- **Validade e nome:** mesma fonte monoespaçada, cor **branco sujo (#E0E0E0)** para contraste, todo em caixa alta.
- **Detalhe escondido:** sob luz UV, aparece o texto "THIS CARD IS A PRIVATE KEY" em letras fantasma atravessando toda a face frontal.

---

### Face Traseira (Verso)

```
┌─────────────────────────────────────────────┐
│                                             │
│  ╔═══════════════════════════════════════╗  │
│  ║ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ║  │
│  ║ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ║  │
│  ║ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ║  │
│  ║ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ║  │
│  ╚═══════════════════════════════════════╝  │
│                                             │
│     ┌─────────────────────────────┐         │
│     │                             │         │
│     │    ASSINATURA DO PORTADOR   │         │
│     │                             │         │
│     └─────────────────────────────┘         │
│                                             │
│                     CVV: ***                │
│                                             │
│  ═══════════════════════════════════════    │
│  Linha de código hexadecimal decorativa     │
│  que atravessa a parte inferior:            │
│  0xDEAD 0xBEEF 0xCAFE 0xBABE 0x5AFE        │
│  ═══════════════════════════════════════    │
│                                             │
│  μBank Black Card — Issued by μBank S.A.    │
│  CNPJ: XX.XXX.XXX/XXXX-XX                   │
│  Customer Service: 0000 000 0000            │
│  This card is property of μBank S.A.        │
│  If found, please destroy and return to:    │
│  [matriz criptográfica de 32 caracteres]    │
│                                             │
└─────────────────────────────────────────────┘
```

**Detalhamento do verso:**

- **Tarja magnética:** personalizada — em vez da faixa preta lisa, uma faixa preta com micro-texto repetido: "μ μ μ μ μ μ μ μ μ μ μ" em relevo quase imperceptível.
- **Painel de assinatura:** fundo com padrão de segurança que repete o símbolo μ em marca d'água. Em vez de dizer "AUTHORIZED SIGNATURE", diz: "IDENTITY HASH".
- **CVV:** 3 dígitos em fonte monoespaçada, com um pequeno ícone de cadeado ao lado.
- **Linha inferior decorativa:** sequência de código hexadecimal (0xDEAD 0xBEEF 0xCAFE 0xBABE 0x5AFE) como easter egg para engenheiros.
- **Rodapé:** informações legais em micro-texto, finalizando com uma matriz criptográfica de 32 caracteres simulada como "endereço de devolução".
- **Detalhe escondido (luz UV):** o símbolo μ gigante em marca d'água fluorescente verde, cobrindo todo o verso do cartão.

---

### Físico do Cartão — Sensação ao Toque

| Característica | Descrição |
| :--- | :--- |
| **Peso** | 17g a 22g (metal), bem mais pesado que cartão plástico (5g) |
| **Temperatura** | Frio ao toque (metal), esquenta lentamente na mão |
| **Textura** | Fosca aveludada, sem impressões digitais visíveis |
| **Som ao cair** | Som metálico seco, como uma moeda pesada caindo em mármore |
| **Gravação** | Número e nome em baixo-relevo com tinta metálica; o símbolo μ em alto-relevo sutil |
| **Borda** | Chanfrada com ângulo de 45°, expondo o metal nu (cor de cobre oxidado ou dourado frio) |

---

### Embalagem do Cartão (Box de Entrega)

```
┌──────────────────────────────────────────────────┐
│                                                  │
│   Caixa preta fosca com o símbolo μ em baixo     │
│   relevo na tampa, preenchido com tinta que      │
│   brilha em verde sob luz UV.                    │
│                                                  │
│   Ao abrir a tampa (que desliza magneticamente): │
│                                                  │
│   Parte interna da tampa:                        │
│   "YOU ARE NOW A NODE IN THE μ NETWORK."         │
│                                                  │
│   Base da caixa:                                 │
│   Cartão em repouso sobre um berço de espuma     │
│   preta cortada a laser. Ao lado, um envelope    │
│   preto com lacre holográfico μ contendo:        │
│     — Carta de boas-vindas                       │
│     — Chave PIX personalizada                    │
│     — Adesivo μ para laptop                      │
│     — Instruções de ativação                     │
│                                                  │
└──────────────────────────────────────────────────┘
```

**Toque final na carta de boas-vindas:**

> *"Este cartão é uma chave privada física. Proteja-o como protegeria sua seed phrase. Nós não podemos recuperar sua identidade — apenas você pode. Bem-vindo ao μBank."*

---

### Prompt para Gerador de Imagem (Render do Cartão)

```
A premium black metal credit card for "μBank", cyber-aggressive cybersecurity aesthetic. The card is black brushed metal, matte finish, with angled chamfered edges exposing cold gold metal. Front face: top left has "μBank" logo in glowing terminal green (#00FF41). Center has an octagonal cryptographic chip with micro-text. Below chip, the Greek letter μ (mu) stylized as a technological rune with a key/padlock, engraved in bas-relief with color-shifting metallic ink (green to gold). Card number in monospaced font (JetBrains Mono), terminal green, with wide spacing. Dark background, dramatic lighting, product photography style, 4K, sharp focus, no plastic look — pure metal and security hardware aesthetic.
```

---

Se quiser, posso gerar variações: **versão Rose Gold**, **versão Transparente** (tipo cartão de acrílico com circuitos visíveis), ou **versão Dual** (cartão físico + cartão virtual com animação de ativação). É só pedir.