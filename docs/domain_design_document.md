# **Domain Design Document**

## Princípios do  μBank
> _**Honestidade em pequenas coisas** não é uma coisa pequena!_ Ditado dinamarquês (citado por James O. Coplien no prefácio do livro Código Limpo)
 
>_As **pequenas coisas são importantes**!_ idem

> _Quem é **fiel no pouco**, também é fiel no muito._ Lc 16:10 (Bíblia Sagrada)

> _... foste **fiel no pouco**, sobre o muito te colocarei;..._ Mt 25:21 (Bíblia Sagrada)

> _Não desprezem **os pequenos começos**, pois o Senhor se alegra ao ver a obra começar..._ Zc 4:10 (Bíblia Sagrada)

>  _Apanhai-me as raposas, **as raposinhas, que fazem mal às vinhas**, porque as nossas vinhas estão em flor._

## 1. O que se sabe publicamente de alguns bancos digitais

### Nubank

Arquitetura pública conhecida:

* **Microservices em larga escala**.
* Backend majoritariamente em **Clojure**.
* Comunicação assíncrona com **Kafka**.
* Banco de dados para dados de alto valor: **Datomic**.
* Infraestrutura com **Kubernetes**.
* Mobile app em **Flutter**.
* Forte influência de **arquitetura hexagonal** e modelos imutáveis. ([Building Nubank][1])

Em 2025, o próprio Nubank relatou operar mais de **4.000 microservices**, processando cerca de **72 bilhões de eventos diários via Kafka**. ([Building Nubank][2])

### Openbank / Santander

Arquitetura pública conhecida:

* Plataforma digital do Santander.
* Uso forte de **cloud**, incluindo AWS para website, mobile app e core banking. ([US Press Center][3])
* Histórico de uso/adoção de **Temenos Core Banking** para expansão internacional. ([Temenos][4])
* Integração recente com infraestrutura bancária do Santander chamada **Gravity** e interface **ODS**. ([Santander][5])
* Modelo mais próximo de **banco digital corporativo**, acoplado a uma grande instituição tradicional.

## 2. Diferença estrutural provável

| Aspecto              | Nubank                                       | Openbank/Santander                                        |
| -------------------- | -------------------------------------------- | --------------------------------------------------------- |
| Origem               | Fintech cloud-native                         | Banco digital de grupo bancário tradicional               |
| Estilo               | Microservices + eventos                      | Plataforma digital + core bancário corporativo            |
| Stack pública        | Clojure, Kafka, Datomic, Kubernetes, Flutter | Cloud, core banking, Gravity, ODS, Temenos historicamente |
| Cultura arquitetural | Produto + domínio + escala                   | Plataforma bancária global + integração institucional     |
| Ponto forte          | Agilidade, eventos, experiência digital      | Conformidade, solidez, integração bancária global         |

## 3. Entidades centrais de domínio para um banco digital

Para o **μBank educativo**, eu começaria com estes bounded contexts:

1. **Identity & Onboarding**

    * `Customer`
    * `Document`
    * `KycProfile`
    * `RiskProfile`
    * `Consent`

2. **Accounts**

    * `BankAccount`
    * `AccountHolder`
    * `Balance`
    * `AccountStatus`

3. **Ledger**

    * `LedgerAccount`
    * `JournalEntry`
    * `Posting`
    * `Transaction`
    * `Money`

4. **Cards**

    * `Card`
    * `CardLimit`
    * `Authorization`
    * `CardTransaction`
    * `Invoice`

5. **Payments**

    * `PixTransfer`
    * `BankSlip`
    * `InternalTransfer`
    * `PaymentOrder`

6. **Credit**

    * `CreditAccount`
    * `Loan`
    * `Installment`
    * `InterestRate`
    * `CreditPolicy`

7. **Education Layer**

    * `LearningGoal`
    * `FinancialChallenge`
    * `BudgetSimulation`
    * `SpendingInsight`
    * `Achievement`

8. **Compliance & Audit**

    * `AuditEvent`
    * `FraudAlert`
    * `RegulatoryReport`
    * `SecurityEvent`

## 4. Esqueleto de Domain Design Document para o μBank

```md
# μBank — Domain Design Document

## 1. Visão do Produto
O μBank é um banco digital educativo voltado ao ensino de educação financeira para jovens, simulando operações bancárias reais em ambiente seguro.

## 2. Inspirações Arquiteturais
- Nubank: microservices, arquitetura orientada a eventos, domínio forte.
- Openbank: plataforma bancária digital integrada a core banking robusto.
- μBank: versão pedagógica, modular, segura e implementável em Rust.

## 3. Bounded Contexts

### 3.1 Identity & Onboarding
Responsável por cadastro, identidade, consentimento e perfil inicial.

Entidades:
- Customer
- Document
- KycProfile
- Consent

### 3.2 Accounts
Responsável por contas digitais e saldo disponível.

Entidades:
- BankAccount
- AccountHolder
- Balance

### 3.3 Ledger
Coração contábil do sistema.

Entidades:
- LedgerAccount
- JournalEntry
- Posting
- Transaction

Regra central:
Toda movimentação financeira deve ser registrada por partidas dobradas.

### 3.4 Payments
Responsável por pagamentos e transferências simuladas.

Entidades:
- PaymentOrder
- PixTransfer
- BankSlip
- InternalTransfer

### 3.5 Cards
Responsável por cartão, autorizações, limites e fatura.

Entidades:
- Card
- Authorization
- CardTransaction
- Invoice

### 3.6 Credit
Responsável por crédito, juros e simulação de empréstimos.

Entidades:
- Loan
- Installment
- InterestRate
- CreditPolicy

### 3.7 Financial Education
Camada pedagógica do μBank.

Entidades:
- LearningGoal
- FinancialChallenge
- BudgetSimulation
- SpendingInsight
- Achievement

## 4. Agregados

### Customer Aggregate
Root: Customer

### Account Aggregate
Root: BankAccount

### Ledger Aggregate
Root: JournalEntry

### Card Aggregate
Root: Card

### Loan Aggregate
Root: Loan

## 5. Eventos de Domínio

- CustomerRegistered
- AccountOpened
- MoneyDeposited
- TransferRequested
- TransferCompleted
- CardAuthorized
- InvoiceClosed
- LoanSimulated
- FinancialChallengeCompleted
- FraudSuspicionRaised

## 6. Serviços de Domínio

- AccountOpeningService
- TransferService
- LedgerPostingService
- CardAuthorizationService
- CreditSimulationService
- FinancialEducationService
- FraudDetectionService

## 7. Regras de Negócio Essenciais

- Uma conta não pode movimentar valor negativo.
- Toda transação deve gerar lançamento no ledger.
- O saldo disponível deriva dos lançamentos contábeis.
- Limite de cartão não é saldo da conta.
- Empréstimos devem explicitar CET, juros, parcelas e custo total.
- Toda operação relevante deve gerar evento de auditoria.

## 8. Arquitetura Sugerida

Frontend:
- App/Web educativo

Backend:
- Rust
- Arquitetura hexagonal
- API REST ou GraphQL
- Event bus interno
- PostgreSQL para persistência inicial
- Ledger imutável

Módulos:
- mubank_identity
- mubank_accounts
- mubank_ledger
- mubank_payments
- mubank_cards
- mubank_credit
- mubank_education
- mubank_audit

## 9. Decisão Arquitetural Inicial

O μBank não deve começar como microservices.
Deve começar como modular monolith em Rust, com fronteiras de domínio bem definidas.
```

## Minha recomendação

Para o seu caso, eu **não copiaria Nubank nem Openbank diretamente**. Eu faria:

**μBank = modular monolith em Rust + DDD + ledger contábil imutável + eventos de domínio internos.**

Depois, se o projeto crescer, alguns módulos podem virar serviços separados. Começar direto com microservices seria pedagogicamente interessante, mas arquiteturalmente caro demais para a primeira versão.

[1]: https://building.nubank.com/the-value-of-canonicity/?utm_source=chatgpt.com "The value of canonicity - Building Nubank"
[2]: https://building.nubank.com/managing-cloud-limits/?utm_source=chatgpt.com "Managing Cloud Limits - Building Nubank"
[3]: https://press.aboutamazon.com/2018/11/santanders-openbank-goes-all-in-on-aws?utm_source=chatgpt.com "Santanders-Openbank-Goes-All-In-on-AWS - US Press Center"
[4]: https://www.temenos.com/success-story/openbank-success-story/?utm_source=chatgpt.com "Temenos Core Banking and Openbank - Success Story"
[5]: https://www.santander.com/en/press-room/features/openbank-us-milestone-in-santander-transformation?utm_source=chatgpt.com "Santander launches Openbank in the US, a significant ..."
