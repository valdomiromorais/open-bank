<!-- #[ptbr] Coisa a lembrar:
 - [Link Text](path)

-->
# MUB Glossário — Linguagem Ubíqua do μBank

$ \mu\$ $

> *Precisão cirúrgica. Zero ambiguidade. Um termo, um significado.*

---

## A

### Account (ptbr: Conta)
Entidade que representa uma conta bancária digital. Contém um identificador único (`AccountId`), um titular (`Customer`), um status (`AccountStatus`) e um saldo (`Balance`) derivado do ledger.

**Regras:**
- Toda conta pertence a exatamente um `Customer`
- O saldo nunca é armazenado — é agregado dos lançamentos contábeis
- Contas em modo LEARNING operam exclusivamente com moeda MUB

### AccountStatus (Status da Conta)
Enum: `Active | Frozen | Closed | PendingVerification`

### AccountHolder (Titular)
Vínculo entre `Account` e `Customer`. Um `Customer` pode ter múltiplas contas.

### AuditEvent (ptbr: Evento de Auditoria)
Registro imutável de uma operação relevante para compliance. Inclui: timestamp, operador, ação, dados da operação, hash do evento anterior (encadeamento).

### Authorization (ptbr: Autorização)
Aprovação ou negação de uma operação (transação, uso de cartão) com base em saldo, limite, perfil de risco e regras de compliance.



---

## B

### BaaS (Banking as a Service)
Modelo de negócio onde o μBank expõe suas funcionalidades bancárias via API para terceiros (ERPs, fintechs parceiras). Inspirado no modelo da Conta Azul. O crate `mu_api` será desenhado para suportar esse modelo com chaves de API e versionamento.

### Balance (ptbr: Saldo)
Valor monetário disponível em uma conta. **Nunca é armazenado** — é computado por agregação (`fold`) dos eventos do ledger.

### BankSlip (ptbr: Boleto)
Ordem de pagamento simulada, com código de barras, vencimento e valor. No modo LEARNING, não tem efeito legal.

### BNPL (Buy Now Pay Later)
Modalidade de crédito que permite ao cliente comprar um bem e pagar parcelado sem cartão de crédito. Popularizado pelo Mercado Pago. Futuro `TransactionKind::Installment` no μBank.

### Bounded Context (ptbr: Contexto Delimitado)
Fronteira explícita de um modelo de domínio. O μBank define 8 contextos: `Identity`, `Accounts`, `Ledger`, `Payments`, `Cards`, `Credit`, `Education`, `Compliance`.

### BudgetSimulation (ptbr: Simulação Orçamentária)
Ferramenta do modo LEARNING para planejar receitas e despesas simuladas com moeda MUB.

---

## C

### Card (ptbr: Cartão)
Entidade que representa um cartão físico ou virtual. Vinculado a uma `Account`. Possui limite próprio, independente do saldo da conta.

### CardLimit (ptbr: Limite do Cartão)
Valor máximo que pode ser autorizado em transações com o cartão. Pode ser global ou por categoria.

### CardTransaction (ptbr: Transação de Cartão)
Movimentação financeira realizada via cartão. Gera lançamento contábil e evento de autorização.

### Cashback (ptbr: Dinheiro de Volta)
Recompensa em MUB creditada ao `Customer` ao completar ações específicas no modo LEARNING (ex: poupar por N dias, completar missões financeiras). Inspirado em PicPay e Stone.

### Compliance (ptbr: Conformidade)
Conjunto de regras e processos para garantir operação dentro da lei (LGPD, BACEN, Open Finance). Inclui KYC, AML, relatórios regulatórios.

### Consent (ptbr: Consentimento)
Registro da autorização explícita do `Customer` para uso de seus dados. Base da LGPD.

### CreditAccount (ptbr: Conta de Crédito)
Linha de crédito associada a um `Customer`. Possui limite, taxa de juros, CET e parcelas.

### Currency (ptbr: Moeda)
Unidade monetária do sistema. Ver enum `Currency` em `mu_core`: `MUB | USD | EUR | BRL`.

### Customer (ptbr: Cliente)
Pessoa física ou jurídica titular de contas no μBank. Contém dados de identidade, documentos, perfil KYC e consentimentos.

### Cibersecurity (ptbr: cibersegurança)

A cibersegurança (ou segurança cibernética) é **a prática de proteger redes, sistemas, dispositivos, aplicações e dados**
contra acessos não autorizados, ataques e danos digitais.
Ela garante a segurança das informações corporativas e pessoais através de tecnologia, processos e políticas


---

## D

### Document (ptbr: Documento)
Representação digital de um documento de identificação (RG, CPF, CNPJ, passaporte). Usado no processo de KYC.

### Domain Event (ptbr: Evento de Domínio)
Ocorrência significativa no domínio, registrada de forma imutável. Ex: `AccountOpened`, `MoneyDeposited`, `TransferCompleted`.

### Double-Entry (ptbr: Partidas Dobradas)
Princípio contábil: toda movimentação financeira gera **dois** lançamentos simultâneos — um débito em uma conta contábil e um crédito em outra. O ledger do μBank segue este princípio.

---

## E

### Education Layer (ptbr: Camada Educacional)
Módulo do modo LEARNING que fornece desafios financeiros, simulações orçamentárias, metas de aprendizado e conquistas gamificadas.

### Event Sourcing (ptbr: Fonte de Eventos)
Padrão arquitetural onde **o estado atual do sistema é derivado da reprodução de todos os eventos passados**.

Event Sourcing (ou Fonte de Eventos) é um **padrão de arquitetura de software** onde as mudanças no estado de uma aplicação
são armazenadas como uma **sequência linear de eventos imutáveis**, em vez de salvar apenas o estado atual em um banco
de dados tradicional.

**O ledger (livro razão) do μBank é preparado para isso**.

#### Vantagens do Event Sourcing
1. **Histórico e Auditoria Perfeitos**:\
   O _**Event Store**_ funciona como um **livro-razão contábil nativo**. Nenhuma informação sobre o passado é descartada.
2. **Consultas Temporais (Time Travel)**:\
   É possível reconstruir o **estado exato do sistema** em qualquer minuto ou dia do passado bastando interromper a
   reprodução dos eventos naquele ponto.
3. **Alta Performance de Escrita**: \
   Inserções no Event Store são operações puras de APPEND (anexar no fim do arquivo/banco). Não existem travas de registros (locks) por atualizações complexas.Depuração Avançada: Se ocorrer um erro em produção, os desenvolvedores podem extrair a lista de eventos e reproduzi-la em um ambiente local de testes para isolar a falha exata.

---

## F

### FinancialChallenge (ptbr: Desafio Financeiro)
Missão gamificada no modo LEARNING. Ex: "Economize 1000 MUB em 30 dias", "Pague 5 boletos em dia".

### FraudAlert (ptbr: Alerta de Fraude)
Notificação gerada por regras ou heurísticas indicando atividade suspeita em uma conta ou transação.

### FraudDetectionService
Serviço de domínio responsável por analisar padrões, aplicar regras e emitir alertas de fraude.

---

## I

### Identity (ptbr: Identidade)
Contexto delimitado responsável por cadastro, autenticação, KYC e consentimento do `Customer`.

### Installment (ptbr: Parcela)
Divisão de um valor total em pagamentos periódicos. Usada em operações de crédito (`Loan`) e fatura de cartão.

### InterestRate (ptbr: Taxa de Juros)
Percentual aplicado sobre saldo devedor ou investimento. No modo LEARNING, usada para ensinar matemática financeira.

### Invoice (ptbr: Fatura)
Resumo mensal das transações de `Card`, com valor total, data de vencimento e parcelas.

### InternalTransfer (ptbr: Transferência Interna)
Movimentação de fundos entre contas do mesmo titular no μBank.

---

## J

### JournalEntry (ptbr: Lançamento Contábil)
Registro individual no ledger. Cada movimentação financeira gera ao menos duas entradas (débito/crédito). Imutável após registrada.

### JournalEntryId
Identificador único (`UUID`) de um `JournalEntry`.

Ver: [UUID](#UUID (Universally Unique Identifier))

---

## K

### KYC (Know Your Customer) (ptbr: Conheça Seu Cliente)
Processo de verificação de identidade do `Customer`. Inclui validação de documentos, biometria e checagem em listas restritivas.

Obs.: É um conjunto de informações e documentos coletados por instituições financeiras para verificar a identidade dos clientes e avaliar os riscos de crimes como lavagem de dinheiro e financiamento ao terrorismo. Esse procedimento é obrigatório por lei e serve para garantir a segurança das transações e o cumprimento de normas regulatórias.
### KycProfile (ptbr: Perfil KYC)
Estado do processo de verificação de um `Customer`. Pode ser: `Pending | Verified | Rejected | NeedsReview`.

---

## L

### LearningGoal (ptbr: Meta de Aprendizado)
Objetivo financeiro definido por um estudante no modo LEARNING. Ex: "Entender juros compostos", "Criar orçamento mensal".

### Ledger (ptbr: Livro-Razão / Contábil)
Registro imutável e cronológico de todas as movimentações financeiras do μBank. Baseado em partidas dobradas. É a **única fonte da verdade** para saldos.

### LedgerAccount (ptbr: Conta Contábil)
Categoria no plano de contas do ledger. Ex: `Ativo.Caixa`, `Passivo.ContaCliente`, `Receita.Juros`. Não confundir com `Account` (conta bancária do cliente).

### Loan (ptbr: Empréstimo)
Operação de crédito com valor principal, taxa de juros, CET, número de parcelas e calendário de pagamento.

---

## M

### MUB (ptbr: Moeda do μBank)
Moeda fiduciária virtual do modo LEARNING. Código ISO 4217 privado: `999`. Símbolo: `µ`. Usada exclusivamente em ambiente sandbox educacional.

### Money (ptbr: Dinheiro)
Tipo valor composto por `amount: Decimal` e `currency: Currency`. Toda operação aritmética valida: mesma moeda, sem _overflow_.

### Modular Monolith (ptbr: Monólito Modular)

Um monólito modular **_é uma arquitetura de software_ onde a aplicação é escrita em uma _única base de código_ 
(o monólito), mas é fortemente dividida em _módulos independentes_ com _limites lógicos bem definidos_**. 
Ele entrega a organização e a coesão dos microsserviços, mas sem a complexidade operacional de rede e infraestrutura.

#### Principais Conceitos
1. **Base de Código Única**: Todo o sistema reside em um único repositório e é compilado e implantado junto.
2. **Módulos de Domínio**: O sistema é **quebrado por áreas de negócio** (ex: Currency, Money, Customer).
Cada módulo possui **sua própria regra de negócio encapsulada** e sabe exatamente o que expor para o resto do sistema.
3. **Baixo Acoplamento**: As regras de um módulo não devem acessar diretamente o banco de dados ou as entranhas de
outro módulo. A comunicação ocorre apenas por interfaces ou APIs bem definidas.

#### Vantagens
1. **Simplicidade Operacional**: Mais fácil de testar, versionar e fazer o deploy, já que você não precisa orquestrar
dezenas de microsserviços na nuvem.
2. **Fácil Evolução**: Se o negócio crescer, os módulos podem ser destacados e transformados em microsserviços ou
serviços autônomos de forma muito mais suave do que partindo de um monólito "espaguete".
3. **Organização de Equipes**: Permite que diferentes equipes fiquem responsáveis por módulos ou domínios isolados,
reduzindo conflitos no código.

### Arquitetura do μBank
o μBank nasce como um **monólito modular**. Um único processo com **fronteiras de domínio** bem definidas (crates),
preparado para **extração futura de microservices**.

Isso permite definir **fronteiras claras** entre os domínios sem a **latência e a complexidade de rede inicial**,
facilitando uma migração futura para **serviços distribuídos**.


### Microsserviços (Arquitetura)
A arquitetura de microsserviços é uma abordagem de desenvolvimento de software em que **uma aplicação grande é dividida
em um conjunto de pequenos serviços independentes e altamente especializados, que se comunicam entre si por meio de APIs**.
Diferente do modelo **tradicional monolítico** — onde todo o sistema roda em um único bloco de código unificado —,
cada microsserviço cuida de apenas uma **_função de negócio_** e possui seu próprio banco de dados isolado.



---

## O

### Open Finance
Sistema de compartilhamento de dados financeiros entre instituições autorizadas pelo Banco Central. Contexto futuro do Interessado 3.

---

## P

### PaymentOrder (ptbr: Ordem de Pagamento)
Instrução para transferir fundos de uma conta para outra (mesmo titular ou terceiros).

### PCI-DSS (Payment Card Industry Data Security Standard)
Conjunto de 12 requisitos de segurança para processamento de dados de cartão. Referência: PagBank mantém conformidade PCI-DSS. O μBank deve documentar quais requisitos se aplicam (Trimestre 4).

### PixTransfer (ptbr: Transferência PIX)
Modalidade de transferência instantânea simulada no modo LEARNING. No futuro, poderá conectar-se à API real do PIX.

### Posting (ptbr: Lançamento)
Ato de registrar uma entrada no ledger. Uma `Transaction` gera múltiplos `Postings` (débitos e créditos).

---

## R

### RegulatoryReport (ptbr: Relatório Regulatório)
Documento gerado para atender exigências de órgãos reguladores (BACEN, CVM). Inclui extrato de movimentações, suspeitas de lavagem, etc.

### RiskProfile (ptbr: Perfil de Risco)
Classificação de um `Customer` quanto ao risco de crédito, fraude ou lavagem de dinheiro. Determinado por regras de compliance.

### Red Team
Um Red Team (equipe vermelha) é um grupo de **especialistas em segurança ofensiva** contratados para atuar como **hackers éticos**.
A função deles é **simular ataques realistas e avançados para testar as defesas de uma organização**, expondo vulnerabilidades
em pessoas, processos e tecnologias antes que criminosos reais as encontrem.

---

## S

### SecurityEvent (ptbr: Evento de Segurança)
Ocorrência relacionada à segurança do sistema: tentativa de acesso não autorizado, alteração de credenciais, detecção de anomalia.

### SpendingInsight (ptbr: Insight de Gastos)
Análise automatizada dos padrões de gasto de um `Customer`, usada na camada educacional para promover conscientização financeira.

---

## T

### Transaction (ptbr: Transação Financeira)
Movimentação de valor entre entidades do μBank. Ex: `Deposit`, `Withdraw`, `Transfer`, `Payment`. Toda transação gera `JournalEntry`(s) no ledger.

**Regras:**
- Uma transação bem-sucedida é **irreversível** — correções são feitas com novas transações (estorno)
- Toda transação tem um tipo, valor, moeda, origem, destino e timestamp
- Transações entre moedas diferentes são barradas em runtime com `MoneyError::CurrencyMismatch`

### Transfer (ptbr: Transferência)
Movimentação de fundos entre duas contas. Pode ser interna (mesmo titular) ou para terceiros.

## U

### UUID (Universally Unique Identifier)

É um identificador padrão de 128 bits ($0$ a $2^{128}-1$) _utilizado no desenvolvimento de **software**_ para
garantir a singularidade de informações em **sistemas distribuídos**, sem a necessidade de uma **coordenação central**.

#### Características Principais
- Singularidade: A probabilidade de dois `UUID`s serem idênticos é tão baixa que é considerada desprezível para fins práticos.
- Estrutura: É representado como uma sequência de 32 dígitos hexadecimais {0, … 9, A, …, F} : a cada dois dígitos
hexadeciamais temos 8 bits, 1 byte, o que significa que temos 16 bytes: 16 * 8 = 128 bits, divididos em cinco grupos]
separados por hífens, no formato:8-4-4-4-12 (exemplo:123e4567-e89b-12d3-a456-426614174000).
- Autonomia: Diferente de chaves primárias (_primary keys_) incrementais em bancos de dados (1, 2, 3...),
o UUID pode ser gerado localmente por uma aplicação, garantindo que o ID será único mesmo quando integrado a outros
sistemas futuramente.

#### Versões Comuns
1. **Versão 1 (Baseada no Tempo)**: Gerado a partir do **carimbo de data/hora (_timestamp_)** do sistema e
do **endereço MAC** do dispositivo.
2. **Versão 4 (Aleatória)**: Gerado quase inteiramente **a partir de números aleatórios** ou pseudo-aleatórios.
**É a versão mais utilizada atualmente devido à sua simplicidade e privacidade**.
3. **Versão 5 (Baseada em Nome/Hash)**: Gerado através do hash (SHA-1) de um nome e um namespace específico.

#### Casos de Uso
1. **Bancos de Dados**: Como chave primária para evitar conflitos de _ID_ durante a fusão de dados de diferentes servidores.
2. **Sistemas Distribuídos**: Identificação de recursos, sessões de usuário e **_transações em arquiteturas de microsserviços_**.
3. **Desenvolvimento Web**: Geração de nomes de arquivos únicos para _uploads_ ou _tokens_ de rastreamento.

### UTC (ptbr: Tempo Universal Coordenado)

O UTC (Tempo Universal Coordenado) é o fuso horário de referência global, baseado em relógios atômicos.
Ele substituiu o antigo GMT (Tempo Médio de Greenwich). Todos os fusos horários do mundo são calculados
como adição ou subtração de horas em relação ao UTC (ex: \(UTC -3\) ou \(UTC +1\)). Para verificar a hora
exata no meridiano de referência ou em sua localização, acerte seus relógios através do Time.is ou consulte
o Time and Date.

No Brasil: A maior parte do país, incluindo Petrolina–PE, está no fuso UTC-3 (3 horas
atrasado em relação ao horário base do UTC).

Em Portugal: O território continental opera no horário UTC
durante o inverno e muda para UTC+1 durante o horário de verão europeu.

---

*Este glossário é vivo — novos termos são adicionados conforme o domínio evolui. Ambiguidade é débito técnico.*
