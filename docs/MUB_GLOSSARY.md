# MUB Glossário — Linguagem Ubíqua do μBank

$ \mu\$ $

> *Precisão cirúrgica. Zero ambiguidade. Um termo, um significado.*

---

## A

### Account (Conta)
Entidade que representa uma conta bancária digital. Contém um identificador único (`AccountId`), um titular (`Customer`), um status (`AccountStatus`) e um saldo (`Balance`) derivado do ledger.

**Regras:**
- Toda conta pertence a exatamente um `Customer`
- O saldo nunca é armazenado — é agregado dos lançamentos contábeis
- Contas em modo LEARNING operam exclusivamente com moeda MUB

### AccountStatus (Status da Conta)
Enum: `Active | Frozen | Closed | PendingVerification`

### AccountHolder (Titular)
Vínculo entre `Account` e `Customer`. Um `Customer` pode ter múltiplas contas.

### AuditEvent (Evento de Auditoria)
Registro imutável de uma operação relevante para compliance. Inclui: timestamp, operador, ação, dados da operação, hash do evento anterior (encadeamento).

### Authorization (Autorização)
Aprovação ou negação de uma operação (transação, uso de cartão) com base em saldo, limite, perfil de risco e regras de compliance.



---

## B

### Balance (Saldo)
Valor monetário disponível em uma conta. **Nunca é armazenado** — é computado por agregação (`fold`) dos eventos do ledger.

### BankSlip (Boleto)
Ordem de pagamento simulada, com código de barras, vencimento e valor. No modo LEARNING, não tem efeito legal.

### Bounded Context (Contexto Delimitado)
Fronteira explícita de um modelo de domínio. O μBank define 8 contextos: `Identity`, `Accounts`, `Ledger`, `Payments`, `Cards`, `Credit`, `Education`, `Compliance`.

### BudgetSimulation (Simulação Orçamentária)
Ferramenta do modo LEARNING para planejar receitas e despesas simuladas com moeda MUB.

---

## C

### Card (Cartão)
Entidade que representa um cartão físico ou virtual. Vinculado a uma `Account`. Possui limite próprio, independente do saldo da conta.

### CardLimit (Limite do Cartão)
Valor máximo que pode ser autorizado em transações com o cartão. Pode ser global ou por categoria.

### CardTransaction (Transação de Cartão)
Movimentação financeira realizada via cartão. Gera lançamento contábil e evento de autorização.

### Compliance (Conformidade)
Conjunto de regras e processos para garantir operação dentro da lei (LGPD, BACEN, Open Finance). Inclui KYC, AML, relatórios regulatórios.

### Consent (Consentimento)
Registro da autorização explícita do `Customer` para uso de seus dados. Base da LGPD.

### CreditAccount (Conta de Crédito)
Linha de crédito associada a um `Customer`. Possui limite, taxa de juros, CET e parcelas.

### Currency (Moeda)
Unidade monetária do sistema. Ver enum `Currency` em `mu_core`: `MUB | USD | EUR | BRL`.

### Customer (Cliente)
Pessoa física ou jurídica titular de contas no μBank. Contém dados de identidade, documentos, perfil KYC e consentimentos.

---

## D

### Document (Documento)
Representação digital de um documento de identificação (RG, CPF, CNPJ, passaporte). Usado no processo de KYC.

### Domain Event (Evento de Domínio)
Ocorrência significativa no domínio, registrada de forma imutável. Ex: `AccountOpened`, `MoneyDeposited`, `TransferCompleted`.

### Double-Entry (Partidas Dobradas)
Princípio contábil: toda movimentação financeira gera **dois** lançamentos simultâneos — um débito em uma conta contábil e um crédito em outra. O ledger do μBank segue este princípio.

---

## E

### Education Layer (Camada Educacional)
Módulo do modo LEARNING que fornece desafios financeiros, simulações orçamentárias, metas de aprendizado e conquistas gamificadas.

### Event Sourcing (Fonte de Eventos)
Padrão arquitetural onde o estado atual do sistema é derivado da reprodução de todos os eventos passados. O ledger do μBank é preparado para isso.

---

## F

### FinancialChallenge (Desafio Financeiro)
Missão gamificada no modo LEARNING. Ex: "Economize 1000 MUB em 30 dias", "Pague 5 boletos em dia".

### FraudAlert (Alerta de Fraude)
Notificação gerada por regras ou heurísticas indicando atividade suspeita em uma conta ou transação.

### FraudDetectionService
Serviço de domínio responsável por analisar padrões, aplicar regras e emitir alertas de fraude.

---

## I

### Identity (Identidade)
Contexto delimitado responsável por cadastro, autenticação, KYC e consentimento do `Customer`.

### Installment (Parcela)
Divisão de um valor total em pagamentos periódicos. Usada em operações de crédito (`Loan`) e fatura de cartão.

### InterestRate (Taxa de Juros)
Percentual aplicado sobre saldo devedor ou investimento. No modo LEARNING, usada para ensinar matemática financeira.

### Invoice (Fatura)
Resumo mensal das transações de `Card`, com valor total, data de vencimento e parcelas.

### InternalTransfer (Transferência Interna)
Movimentação de fundos entre contas do mesmo titular no μBank.

---

## J

### JournalEntry (Lançamento Contábil)
Registro individual no ledger. Cada movimentação financeira gera ao menos duas entradas (débito/crédito). Imutável após registrada.

### JournalEntryId
Identificador único (`UUID`) de um `JournalEntry`.

---

## K

### KYC (Know Your Customer ou Conheça Seu Cliente)
Processo de verificação de identidade do `Customer`. Inclui validação de documentos, biometria e checagem em listas restritivas.

Obs.: É um conjunto de informações e documentos coletados por instituições financeiras para verificar a identidade dos clientes e avaliar os riscos de crimes como lavagem de dinheiro e financiamento ao terrorismo. Esse procedimento é obrigatório por lei e serve para garantir a segurança das transações e o cumprimento de normas regulatórias.
### KycProfile (Perfil KYC)
Estado do processo de verificação de um `Customer`. Pode ser: `Pending | Verified | Rejected | NeedsReview`.

---

## L

### LearningGoal (Meta de Aprendizado)
Objetivo financeiro definido por um estudante no modo LEARNING. Ex: "Entender juros compostos", "Criar orçamento mensal".

### Ledger (Livro-Razão / Contábil)
Registro imutável e cronológico de todas as movimentações financeiras do μBank. Baseado em partidas dobradas. É a **única fonte da verdade** para saldos.

### LedgerAccount (Conta Contábil)
Categoria no plano de contas do ledger. Ex: `Ativo.Caixa`, `Passivo.ContaCliente`, `Receita.Juros`. Não confundir com `Account` (conta bancária do cliente).

### Loan (Empréstimo)
Operação de crédito com valor principal, taxa de juros, CET, número de parcelas e calendário de pagamento.

---

## M

### MUB (Moeda do μBank)
Moeda fiduciária virtual do modo LEARNING. Código ISO 4217 privado: `999`. Símbolo: `µ`. Usada exclusivamente em ambiente sandbox educacional.

### Money (Dinheiro)
Tipo valor composto por `amount: Decimal` e `currency: Currency`. Toda operação aritmética valida: mesma moeda, sem _overflow_.

### Modular Monolith (Monólito Modular)
Arquitetura do μBank: um único processo com fronteiras de domínio bem definidas (crates), preparado para extração futura de microservices.

---

## O

### Open Finance
Sistema de compartilhamento de dados financeiros entre instituições autorizadas pelo Banco Central. Contexto futuro do Interessado 3.

---

## P

### PaymentOrder (Ordem de Pagamento)
Instrução para transferir fundos de uma conta para outra (mesmo titular ou terceiros).

### PixTransfer (Transferência PIX)
Modalidade de transferência instantânea simulada no modo LEARNING. No futuro, poderá conectar-se à API real do PIX.

### Posting (Lançamento)
Ato de registrar uma entrada no ledger. Uma `Transaction` gera múltiplos `Postings` (débitos e créditos).

---

## R

### RegulatoryReport (Relatório Regulatório)
Documento gerado para atender exigências de órgãos reguladores (BACEN, CVM). Inclui extrato de movimentações, suspeitas de lavagem, etc.

### RiskProfile (Perfil de Risco)
Classificação de um `Customer` quanto ao risco de crédito, fraude ou lavagem de dinheiro. Determinado por regras de compliance.

---

## S

### SecurityEvent (Evento de Segurança)
Ocorrência relacionada à segurança do sistema: tentativa de acesso não autorizado, alteração de credenciais, detecção de anomalia.

### SpendingInsight (Insight de Gastos)
Análise automatizada dos padrões de gasto de um `Customer`, usada na camada educacional para promover conscientização financeira.

---

## T

### Transaction (Transação Financeira)
Movimentação de valor entre entidades do μBank. Ex: `Deposit`, `Withdraw`, `Transfer`, `Payment`. Toda transação gera `JournalEntry`(s) no ledger.

**Regras:**
- Uma transação bem-sucedida é **irreversível** — correções são feitas com novas transações (estorno)
- Toda transação tem um tipo, valor, moeda, origem, destino e timestamp
- Transações entre moedas diferentes são barradas em runtime com `MoneyError::CurrencyMismatch`

### Transfer (Transferência)
Movimentação de fundos entre duas contas. Pode ser interna (mesmo titular) ou para terceiros.

---

*Este glossário é vivo — novos termos são adicionados conforme o domínio evolui. Ambiguidade é débito técnico.*
