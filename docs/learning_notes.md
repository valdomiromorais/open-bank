# Learning Note
> Anotar é preciso

---



## Rust Notes

1. Arrays ("[ ]") e string em Rust índexam a partir de 0 

---

## Financial Learning Notes

### UUID
   
UUID significa **Universally Unique Identifier** (Identificador Único Universal). É um valor numérico de **_128 bits**_ utilizado para identificar de forma exclusiva qualquer tipo de informação ou objeto em sistemas de computação.
A principal característica do UUID é permitir que **sistemas distribuídos gerem identificadores locais sem a necessidade de uma autoridade central de coordenação**.
Isso significa que dois sistemas independentes podem criar UUIDs simultaneamente com **uma probabilidade matematicamente desprezível de gerarem o mesmo código**.

### ---

---

## Pequenas coisas

Segurança militar não é um feature — é uma **disciplina de cada decisão trivial**. O cliente se sente seguro não porque um banner diz "seguro", mas porque o sistema **não falha de formas que ele consegue perceber**.

Aqui estão as "pequenas coisas" que vão materializar isso no código do μBank:

### 1. Zero débito técnico de domínio
- Operações entre moedas diferentes = **não compila ou não executa** (já temos com `MoneyError::CurrencyMismatch`)
- Saldo nunca é armazenado como campo — é **sempre derivado do ledger** (já está no glossário). Se o banco de dados corromper, o saldo correto é recuperável.
- Transferência só completa se **débito E crédito** forem registrados (propriedade ACID na mão)

### 2. A aritmética não mente
- `Decimal` (i128, 28 dígitos) — sem `f64` em lugar nenhum. `0.1 + 0.2` nunca será `0.30000000000000004`
- Toda operação usa `checked_add`/`checked_sub` — overflow vira `Err`, não silêncio
- Toda função financeira exige **moeda explícita** — nenhum "default USD" escondido

### 3. O ledger é imutável e auditável
- Nada se apaga. Erro de lançamento? **Estorno** (nova transação), nunca `UPDATE` ou `DELETE`
- Cada evento carrega o hash do anterior — cadeia de integridade como blockchain, sem blockchain
- Toda transação acima de um limiar gera alerta de auditoria (já previsto no roadmap semana 15-16)

### 4. Autenticação e identidade
- Senha com `argon2` (nunca hash rápido como SHA-256 para senhas — já previsto semana 7-8)
- KYC desde o começo: `Customer` não existe sem documento verificado
- Consentimento LGPD: nenhum dado é processado sem registro explícito

### 5. O cliente sente sem ver
- O CLI ou API nunca expõe `500 Internal Server Error` com stack trace
- Logs não vazam dados sensíveis (CPF, senha, saldo) — mesmo em debug
- Downtime planejado é o único downtime — o sistema se recusa a operar inconsistente

### 6. Paranoia produtiva (C/C++ comparisons)
- Os projetos de comparação não são exercício acadêmico — são **provas reais** de que Rust nos protege de classes inteiras de bugs que C/C++ permitem (use-after-free, data races, buffer overflow)
- Cada comparação vira um capítulo no relatório de segurança

---

**TL;DR:** O cliente do μBank não precisa ser especialista em segurança pra confiar. A confiança vem de **ausência de surpresas**. O sistema faz sempre o que deveria fazer, nunca o que não deveria, e quando algo errado acontece, **a culpa nunca é de um bug de domínio** — é de um evento que o sistema registrou e pode provar.