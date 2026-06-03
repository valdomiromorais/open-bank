# Learning Note
> Anotar é preciso

---



## Rust Notes

### Arrays ("[ ]") e string
1. em Rust índexam a partir de 0


### RAII (Resource Acquisition Is Initialization)
RAII é um padrão onde o **ciclo de vida de um recurso** (como memória, arquivos ou locks) **está estritamente ligado ao
escopo de uma variável**. Quando a variável sai de escopo, o compilador libera automaticamente o recurso, eliminando
vazamentos de memória.

O princípio central do RAII é **garantir que a inicialização de um recurso ocorra no momento da criação da variável**, e
a **liberação ocorra automaticamente no momento da destruição**. Em Rust, isso é gerenciado pelo sistema de **_Ownership_**
(Propriedade) e pelo **_Borrow Checker_** (Verificador de empréstimo).

Aquisição: Quando você cria ou atribui um valor a uma variável na stack (ou ponteiro inteligente na heap, como Box),
ela (a variável) assume a propriedade (_ownership_) do recurso.

Liberação: Assim que a variável atinge o fim do seu escopo, o **Rust chama automaticamente o destrutor (destructor) do objeto**.

### Variable Shadowing (ou sombreamento de variáveis)
Em Rust é a prática de **declarar uma nova variável com o mesmo nome de uma variável existente**. A nova variável
"esconde" a antiga dentro do escopo, permitindo alterar valores e tipos **sem precisar que a variável seja mutável**.

Para criar uma variável "shadowed", **basta usar a palavra-chave let novamente com o mesmo nome da variável original**.

Em Rust, **variáveis mutáveis (`let mut`) não podem ter seus tipos alterados após a declaração**. Com o shadowing,
você pode **reaproveitar o nome enquanto transforma o dado em outro tipo**.

```rust
fn main() {
    let espaco = "   "; // espaco é uma String/&str
    let espaco = espaco.len(); // espaco agora é um inteiro (usize)

    println!("O número de espaços é: {}", espaco);
}
```

#### Sombreamento em Escopo Local (Blocos)
Você pode declarar uma variável dentro de um bloco `{}`. Quando esse bloco terminar, a variável sombreada é descartada
e a original volta a ser acessível.

```rust
fn main() {
    let x = 5;
    {
        let x = x * 2; // x é 10 dentro deste bloco
        println!("Valor de x no bloco: {}", x);
    } // Fim do bloco, x sombreado é destruído

    println!("Valor de x fora do bloco: {}", x); // Imprime 5
}
```

#### Shadowing vs. Mutabilidade (mut)
Alguns iniciantes confundem shadowing com mutabilidade, mas são conceitos bem diferentes:

**Mutabilidade** (let mut): A variável permanece sendo a mesma na memória, mas **seu valor pode ser alterado**. **O tipo do
dado não pode mudar**.

**Shadowing** (let): Você está criando uma nova variável. Isso permite alterar o valor, alterar o tipo, e até mesmo
transformá-la de volta em imutável após reatribuí-la.

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