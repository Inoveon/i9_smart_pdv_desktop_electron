# I9 Smart PDV Web - Sistema PDV para Postos de Combustíveis

## Visão Geral

Sistema de Ponto de Venda (PDV) moderno, focado em **teclado-first** para máxima velocidade operacional em postos de combustíveis. Desenvolvido com as mesmas tecnologias do projeto ECTM.

---

## Stack Tecnológico

### Frontend
| Tecnologia | Versão | Propósito |
|------------|--------|-----------|
| Next.js | 16.x | Meta Framework React |
| React | 19.x | Biblioteca UI |
| TypeScript | 5.x | Tipagem estática |
| Tailwind CSS | 4.x | Estilização |
| shadcn/ui | latest | Componentes base |
| Radix UI | latest | Primitivas acessíveis |
| Zustand | latest | Gerenciamento de estado |
| React Hook Form | 7.x | Formulários |
| Zod | 4.x | Validação |
| react-hotkeys-hook | latest | Atalhos de teclado |
| Framer Motion | 12.x | Animações |
| Lucide React | latest | Ícones |
| IndexedDB (Dexie.js) | latest | Storage offline |

### Backend
| Tecnologia | Versão | Propósito |
|------------|--------|-----------|
| Node.js | 18+ | Runtime |
| Express.js | 4.x | Framework HTTP |
| TypeScript | 5.x | Tipagem estática |
| Prisma | 6.x | ORM |
| PostgreSQL | 15+ | Banco de dados |
| Zod | 3.x | Validação |
| JWT | latest | Autenticação |
| bcryptjs | latest | Hash de senhas |
| Helmet | latest | Segurança HTTP |
| CORS | latest | Cross-Origin |
| Rate Limit | latest | Proteção |

### Mobile (React Native / Expo)
| Tecnologia | Versão | Propósito |
|------------|--------|-----------|
| React Native | 0.76.x | Framework mobile |
| Expo | 52.x | Tooling e build |
| TypeScript | 5.x | Tipagem estática |
| NativeWind | 4.x | Tailwind para RN |
| Zustand | latest | Gerenciamento de estado |
| React Hook Form | 7.x | Formulários |
| Zod | 4.x | Validação |
| Expo Router | latest | Navegação |
| MMKV | latest | Storage local rápido |
| React Query | latest | Cache de API |

### Integrações Futuras
| Tecnologia | Propósito |
|------------|-----------|
| WebUSB/WebSerial | Impressora térmica, display cliente |
| SAT/NFC-e | Emissão fiscal |
| TEF | Pagamento cartão |
| PIX API | Pagamento instantâneo |
| Automação de bombas | SASC, Gilbarco, Wayne |
| Bluetooth | Impressora térmica mobile |
| NFC | Leitura de cartões/tags |

---

## Arquitetura do Sistema

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND (Next.js)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   PDV UI    │  │   Admin UI  │  │   Relatórios UI         │  │
│  │ (Teclado)   │  │  (Gestão)   │  │   (Dashboards)          │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│                            │                                      │
│  ┌─────────────────────────┴──────────────────────────────────┐ │
│  │                    Zustand Store                            │ │
│  │  • Venda atual  • Carrinho  • Caixa  • Configurações       │ │
│  └────────────────────────────────────────────────────────────┘ │
│                            │                                      │
│  ┌─────────────────────────┴──────────────────────────────────┐ │
│  │              IndexedDB (Offline Storage)                    │ │
│  │  • Vendas pendentes  • Cache produtos  • Sync queue        │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                        API REST (HTTPS)
                              │
┌─────────────────────────────────────────────────────────────────┐
│                         BACKEND (Express)                        │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                      Routes Layer                           │ │
│  │  /auth  /vendas  /produtos  /caixa  /bombas  /relatorios   │ │
│  └────────────────────────────────────────────────────────────┘ │
│                            │                                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Services Layer                           │ │
│  │  VendaService  CaixaService  BombaService  FiscalService   │ │
│  └────────────────────────────────────────────────────────────┘ │
│                            │                                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                     Prisma ORM                              │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────┐
│                      PostgreSQL Database                         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Estrutura de Pastas

### Frontend
```
frontend/
├── src/
│   ├── app/                          # Next.js App Router
│   │   ├── (pdv)/                    # Grupo: Telas do PDV
│   │   │   ├── layout.tsx            # Layout PDV (fullscreen)
│   │   │   ├── page.tsx              # Tela principal de vendas
│   │   │   ├── abertura/page.tsx     # Abertura de caixa
│   │   │   └── fechamento/page.tsx   # Fechamento de caixa
│   │   │
│   │   ├── (admin)/                  # Grupo: Administração
│   │   │   ├── layout.tsx
│   │   │   ├── dashboard/page.tsx
│   │   │   ├── produtos/page.tsx
│   │   │   ├── funcionarios/page.tsx
│   │   │   ├── bombas/page.tsx
│   │   │   ├── relatorios/page.tsx
│   │   │   └── configuracoes/page.tsx
│   │   │
│   │   ├── (auth)/                   # Grupo: Autenticação
│   │   │   ├── login/page.tsx
│   │   │   └── layout.tsx
│   │   │
│   │   ├── api/                      # API Routes (BFF)
│   │   │   └── sync/route.ts         # Sincronização offline
│   │   │
│   │   ├── layout.tsx
│   │   ├── globals.css
│   │   └── not-found.tsx
│   │
│   ├── components/
│   │   ├── ui/                       # Componentes base (shadcn)
│   │   │   ├── button.tsx
│   │   │   ├── input.tsx
│   │   │   ├── card.tsx
│   │   │   ├── modal.tsx
│   │   │   ├── toast.tsx
│   │   │   ├── kbd.tsx               # Componente de tecla
│   │   │   ├── shortcut-hint.tsx     # Dica de atalho
│   │   │   └── index.ts
│   │   │
│   │   ├── pdv/                      # Componentes específicos PDV
│   │   │   ├── VendaPanel.tsx        # Painel principal de venda
│   │   │   ├── ItemList.tsx          # Lista de itens da venda
│   │   │   ├── TotalDisplay.tsx      # Display de totais
│   │   │   ├── BombaSelector.tsx     # Seletor de bomba
│   │   │   ├── ProdutoSearch.tsx     # Busca de produtos
│   │   │   ├── PaymentModal.tsx      # Modal de pagamento
│   │   │   ├── QuantityInput.tsx     # Input de quantidade
│   │   │   ├── FrentistaBadge.tsx    # Badge do frentista
│   │   │   ├── CaixaStatus.tsx       # Status do caixa
│   │   │   ├── ShortcutBar.tsx       # Barra de atalhos
│   │   │   ├── AbastecimentoCard.tsx # Card de abastecimento
│   │   │   └── ClienteInfo.tsx       # Informações do cliente
│   │   │
│   │   ├── admin/                    # Componentes admin
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Header.tsx
│   │   │   ├── StatsCard.tsx
│   │   │   └── DataTable.tsx
│   │   │
│   │   └── layout/
│   │       ├── PDVLayout.tsx
│   │       └── AdminLayout.tsx
│   │
│   ├── stores/                       # Zustand stores
│   │   ├── useVendaStore.ts          # Estado da venda atual
│   │   ├── useCaixaStore.ts          # Estado do caixa
│   │   ├── useCarrinhoStore.ts       # Itens do carrinho
│   │   ├── useConfigStore.ts         # Configurações
│   │   └── useSyncStore.ts           # Estado de sincronização
│   │
│   ├── hooks/                        # Custom hooks
│   │   ├── useKeyboardNavigation.ts  # Navegação por teclado
│   │   ├── usePDVShortcuts.ts        # Atalhos do PDV
│   │   ├── useOfflineSync.ts         # Sincronização offline
│   │   ├── useBomba.ts               # Integração com bombas
│   │   └── usePrinter.ts             # Integração impressora
│   │
│   ├── lib/
│   │   ├── api.ts                    # Cliente API
│   │   ├── utils.ts                  # Utilitários
│   │   ├── validations.ts            # Schemas Zod
│   │   ├── db.ts                     # IndexedDB (Dexie)
│   │   ├── shortcuts.ts              # Mapa de atalhos
│   │   └── format.ts                 # Formatadores (moeda, etc)
│   │
│   ├── types/
│   │   ├── venda.ts
│   │   ├── produto.ts
│   │   ├── pagamento.ts
│   │   ├── bomba.ts
│   │   └── caixa.ts
│   │
│   └── middleware.ts
│
├── public/
│   ├── sounds/                       # Sons de feedback
│   │   ├── beep.mp3
│   │   ├── success.mp3
│   │   └── error.mp3
│   └── ...
│
├── package.json
├── tsconfig.json
├── tailwind.config.ts
├── next.config.ts
└── .env.example
```

### Backend
```
backend/
├── src/
│   ├── config/
│   │   ├── database.ts
│   │   ├── auth.ts
│   │   └── fiscal.ts
│   │
│   ├── controllers/
│   │   ├── auth.controller.ts
│   │   ├── venda.controller.ts
│   │   ├── produto.controller.ts
│   │   ├── caixa.controller.ts
│   │   ├── bomba.controller.ts
│   │   ├── pagamento.controller.ts
│   │   ├── funcionario.controller.ts
│   │   ├── cliente.controller.ts
│   │   ├── relatorio.controller.ts
│   │   └── sync.controller.ts
│   │
│   ├── services/
│   │   ├── auth.service.ts
│   │   ├── venda.service.ts
│   │   ├── produto.service.ts
│   │   ├── caixa.service.ts
│   │   ├── bomba.service.ts
│   │   ├── pagamento.service.ts
│   │   ├── funcionario.service.ts
│   │   ├── cliente.service.ts
│   │   ├── fiscal.service.ts         # SAT/NFC-e
│   │   ├── tef.service.ts            # Pagamentos cartão
│   │   ├── pix.service.ts            # Pagamentos PIX
│   │   ├── impressao.service.ts      # Impressão cupons
│   │   ├── relatorio.service.ts
│   │   ├── sync.service.ts
│   │   └── auditoria.service.ts
│   │
│   ├── routes/
│   │   ├── auth.routes.ts
│   │   ├── venda.routes.ts
│   │   ├── produto.routes.ts
│   │   ├── caixa.routes.ts
│   │   ├── bomba.routes.ts
│   │   ├── pagamento.routes.ts
│   │   ├── funcionario.routes.ts
│   │   ├── cliente.routes.ts
│   │   ├── relatorio.routes.ts
│   │   ├── sync.routes.ts
│   │   ├── health.routes.ts
│   │   └── index.ts
│   │
│   ├── middlewares/
│   │   ├── auth.middleware.ts
│   │   ├── validation.middleware.ts
│   │   ├── caixa.middleware.ts       # Verifica caixa aberto
│   │   └── auditoria.middleware.ts
│   │
│   ├── schemas/                      # Zod schemas
│   │   ├── venda.schema.ts
│   │   ├── produto.schema.ts
│   │   ├── caixa.schema.ts
│   │   └── pagamento.schema.ts
│   │
│   ├── types/
│   │   └── index.ts
│   │
│   ├── utils/
│   │   ├── fiscal.ts
│   │   ├── format.ts
│   │   └── validators.ts
│   │
│   ├── prisma/
│   │   ├── schema.prisma
│   │   ├── migrations/
│   │   └── seed.ts
│   │
│   └── index.ts
│
├── package.json
├── tsconfig.json
└── .env.example
```

### Mobile (React Native / Expo)
```
mobile/
├── src/
│   ├── app/                          # Expo Router (file-based)
│   │   ├── (auth)/                   # Grupo: Autenticação
│   │   │   ├── login.tsx
│   │   │   └── pin.tsx               # Login por PIN
│   │   │
│   │   ├── (tabs)/                   # Grupo: Navegação principal
│   │   │   ├── _layout.tsx           # Tab navigator
│   │   │   ├── index.tsx             # Home/Dashboard
│   │   │   ├── vendas.tsx            # PDV móvel
│   │   │   ├── bombas.tsx            # Status das bombas
│   │   │   └── perfil.tsx            # Perfil do usuário
│   │   │
│   │   ├── venda/                    # Fluxo de venda
│   │   │   ├── [id].tsx              # Detalhes da venda
│   │   │   └── nova.tsx              # Nova venda
│   │   │
│   │   ├── abastecimento/
│   │   │   └── [id].tsx              # Detalhes do abastecimento
│   │   │
│   │   ├── pagamento/
│   │   │   ├── index.tsx             # Seleção de pagamento
│   │   │   ├── dinheiro.tsx
│   │   │   ├── cartao.tsx
│   │   │   └── pix.tsx
│   │   │
│   │   ├── _layout.tsx               # Root layout
│   │   └── +not-found.tsx
│   │
│   ├── components/
│   │   ├── ui/                       # Componentes base
│   │   │   ├── Button.tsx
│   │   │   ├── Input.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Modal.tsx
│   │   │   ├── Badge.tsx
│   │   │   ├── Spinner.tsx
│   │   │   └── index.ts
│   │   │
│   │   ├── pdv/                      # Componentes PDV mobile
│   │   │   ├── BombaCard.tsx         # Card da bomba
│   │   │   ├── AbastecimentoItem.tsx
│   │   │   ├── ProdutoItem.tsx
│   │   │   ├── TotalBar.tsx          # Barra inferior com total
│   │   │   ├── PaymentSelector.tsx
│   │   │   ├── QuantitySelector.tsx
│   │   │   └── QRCodeScanner.tsx     # Scanner de QR/Barcode
│   │   │
│   │   └── common/
│   │       ├── Header.tsx
│   │       ├── TabBar.tsx
│   │       └── EmptyState.tsx
│   │
│   ├── stores/                       # Zustand stores (compartilhado com web)
│   │   ├── useVendaStore.ts
│   │   ├── useCaixaStore.ts
│   │   ├── useAuthStore.ts
│   │   └── useSyncStore.ts
│   │
│   ├── hooks/
│   │   ├── useApi.ts                 # Hook para chamadas API
│   │   ├── useAuth.ts
│   │   ├── useSync.ts                # Sincronização offline
│   │   ├── useBluetooth.ts           # Impressora Bluetooth
│   │   └── useNFC.ts                 # Leitura NFC
│   │
│   ├── lib/
│   │   ├── api.ts                    # Cliente API
│   │   ├── storage.ts                # MMKV wrapper
│   │   ├── utils.ts
│   │   └── format.ts
│   │
│   ├── types/
│   │   └── index.ts
│   │
│   └── assets/
│       ├── images/
│       └── fonts/
│
├── app.json                          # Expo config
├── package.json
├── tsconfig.json
├── tailwind.config.js                # NativeWind
└── .env.example
```

#### Funcionalidades Mobile

**Frentista (App Pista)**
- Visualizar bombas e status
- Registrar abastecimentos manuais
- Identificar cliente/veículo
- Informar placa e hodômetro

**Operador (App PDV Móvel)**
- PDV completo em tablet
- Vendas de loja
- Recebimentos
- Consulta de preços

**Gerente (App Gestão)**
- Dashboard de vendas
- Acompanhamento em tempo real
- Relatórios básicos
- Notificações de alertas

---

## Modelo de Dados (Prisma Schema)

```prisma
// ============================================
// I9 SMART PDV WEB - SCHEMA DO BANCO DE DADOS
// Posto de Combustíveis
// ============================================

generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

// ============================================
// ENUMS
// ============================================

enum Perfil {
  ADMIN           // Administrador do sistema
  GERENTE         // Gerente do posto
  OPERADOR        // Operador de caixa
  FRENTISTA       // Frentista (apenas abastecimentos)
}

enum StatusCaixa {
  ABERTO
  FECHADO
  SANGRIA         // Em processo de sangria
  SUPRIMENTO      // Em processo de suprimento
}

enum StatusVenda {
  EM_ANDAMENTO
  FINALIZADA
  CANCELADA
  PENDENTE_SYNC   // Aguardando sincronização
}

enum TipoPagamento {
  DINHEIRO
  PIX
  CARTAO_DEBITO
  CARTAO_CREDITO
  VOUCHER_FROTA   // Vale combustível frota
  FIADO           // Venda fiado (cliente cadastrado)
  MISTO           // Múltiplas formas
}

enum StatusPagamento {
  PENDENTE
  APROVADO
  RECUSADO
  CANCELADO
  ESTORNADO
}

enum TipoProduto {
  COMBUSTIVEL
  LUBRIFICANTE
  ADITIVO
  LOJA            // Produtos de conveniência
  SERVICO         // Lavagem, calibragem, etc
}

enum UnidadeMedida {
  LITRO           // L
  UNIDADE         // UN
  QUILO           // KG
  SERVICO         // SV
}

enum StatusBomba {
  LIVRE
  EM_ABASTECIMENTO
  BLOQUEADA
  MANUTENCAO
  OFFLINE
}

enum TipoMovimentoCaixa {
  ABERTURA
  VENDA
  SANGRIA
  SUPRIMENTO
  FECHAMENTO
  ESTORNO
}

enum StatusAbastecimento {
  EM_ANDAMENTO
  CONCLUIDO
  CANCELADO
  PENDENTE        // Aguardando processamento no PDV
}

// ============================================
// USUÁRIOS E AUTENTICAÇÃO
// ============================================

model Usuario {
  id            String    @id @default(uuid())
  nome          String
  email         String    @unique
  cpf           String    @unique
  senha         String    // Hash bcrypt
  perfil        Perfil    @default(OPERADOR)
  ativo         Boolean   @default(true)
  pin           String?   // PIN para acesso rápido (4-6 dígitos)

  // Timestamps
  criadoEm      DateTime  @default(now()) @map("criado_em")
  atualizadoEm  DateTime  @updatedAt @map("atualizado_em")
  ultimoAcesso  DateTime? @map("ultimo_acesso")

  // Relacionamentos
  caixas              Caixa[]
  vendasOperador      Venda[]           @relation("OperadorVenda")
  vendasFrentista     Venda[]           @relation("FrentistaVenda")
  abastecimentos      Abastecimento[]
  movimentosCaixa     MovimentoCaixa[]
  logsAuditoria       LogAuditoria[]

  @@map("usuarios")
}

// ============================================
// CONFIGURAÇÕES DO POSTO
// ============================================

model Posto {
  id              String    @id @default(uuid())
  razaoSocial     String    @map("razao_social")
  nomeFantasia    String    @map("nome_fantasia")
  cnpj            String    @unique
  inscricaoEstadual String? @map("inscricao_estadual")

  // Endereço
  endereco        String
  numero          String
  complemento     String?
  bairro          String
  cidade          String
  estado          String    @db.Char(2)
  cep             String

  // Contato
  telefone        String?
  email           String?

  // Fiscal
  regimeTributario  String?   @map("regime_tributario") // Simples, Lucro Presumido, etc
  crt               Int?      // Código Regime Tributário

  // Configurações
  configFiscal    Json?     @map("config_fiscal") // Configurações SAT/NFC-e
  configTef       Json?     @map("config_tef")    // Configurações TEF
  configPix       Json?     @map("config_pix")    // Configurações PIX

  criadoEm        DateTime  @default(now()) @map("criado_em")
  atualizadoEm    DateTime  @updatedAt @map("atualizado_em")

  // Relacionamentos
  bombas          Bomba[]
  tanques         Tanque[]
  caixas          Caixa[]

  @@map("postos")
}

// ============================================
// INFRAESTRUTURA DO POSTO
// ============================================

model Tanque {
  id              String    @id @default(uuid())
  postoId         String    @map("posto_id")
  numero          Int       // Número do tanque
  capacidade      Float     // Capacidade em litros
  combustivelId   String    @map("combustivel_id")
  nivelAtual      Float?    @map("nivel_atual") // Estoque atual

  criadoEm        DateTime  @default(now()) @map("criado_em")
  atualizadoEm    DateTime  @updatedAt @map("atualizado_em")

  // Relacionamentos
  posto           Posto     @relation(fields: [postoId], references: [id])
  combustivel     Produto   @relation(fields: [combustivelId], references: [id])
  bicos           Bico[]

  @@unique([postoId, numero])
  @@map("tanques")
}

model Bomba {
  id              String       @id @default(uuid())
  postoId         String       @map("posto_id")
  numero          Int          // Número da bomba (1, 2, 3...)
  identificador   String?      // ID no sistema de automação
  status          StatusBomba  @default(LIVRE)

  criadoEm        DateTime     @default(now()) @map("criado_em")
  atualizadoEm    DateTime     @updatedAt @map("atualizado_em")

  // Relacionamentos
  posto           Posto        @relation(fields: [postoId], references: [id])
  bicos           Bico[]
  abastecimentos  Abastecimento[]

  @@unique([postoId, numero])
  @@map("bombas")
}

model Bico {
  id              String    @id @default(uuid())
  bombaId         String    @map("bomba_id")
  tanqueId        String    @map("tanque_id")
  numero          Int       // Número do bico na bomba
  encerrante      Float     @default(0) // Leitura do encerrante

  criadoEm        DateTime  @default(now()) @map("criado_em")
  atualizadoEm    DateTime  @updatedAt @map("atualizado_em")

  // Relacionamentos
  bomba           Bomba     @relation(fields: [bombaId], references: [id])
  tanque          Tanque    @relation(fields: [tanqueId], references: [id])
  abastecimentos  Abastecimento[]

  @@unique([bombaId, numero])
  @@map("bicos")
}

// ============================================
// PRODUTOS
// ============================================

model CategoriaProduto {
  id          String    @id @default(uuid())
  nome        String
  descricao   String?
  ordem       Int       @default(0)
  ativo       Boolean   @default(true)

  criadoEm    DateTime  @default(now()) @map("criado_em")
  atualizadoEm DateTime @updatedAt @map("atualizado_em")

  produtos    Produto[]

  @@map("categorias_produto")
}

model Produto {
  id              String          @id @default(uuid())
  codigo          String          @unique // Código interno / barras
  codigoBarras    String?         @unique @map("codigo_barras")
  nome            String
  descricao       String?
  tipo            TipoProduto
  categoriaId     String?         @map("categoria_id")
  unidade         UnidadeMedida   @default(UNIDADE)

  // Preços
  precoVenda      Decimal         @map("preco_venda") @db.Decimal(10, 4)
  precoCusto      Decimal?        @map("preco_custo") @db.Decimal(10, 4)
  margemLucro     Decimal?        @map("margem_lucro") @db.Decimal(5, 2)

  // Fiscal
  ncm             String?         // Nomenclatura Comum do Mercosul
  cest            String?         // Código Especificador da Substituição Tributária
  cfop            String?         // Código Fiscal de Operações
  cst             String?         // Código de Situação Tributária
  aliquotaIcms    Decimal?        @map("aliquota_icms") @db.Decimal(5, 2)
  aliquotaPis     Decimal?        @map("aliquota_pis") @db.Decimal(5, 4)
  aliquotaCofins  Decimal?        @map("aliquota_cofins") @db.Decimal(5, 4)

  // Controle
  ativo           Boolean         @default(true)
  controlaEstoque Boolean         @default(true) @map("controla_estoque")
  estoqueAtual    Float?          @map("estoque_atual")
  estoqueMinimo   Float?          @map("estoque_minimo")

  // Atalho no PDV
  atalhoPdv       String?         @map("atalho_pdv") // Ex: "F3" para gasolina
  corPdv          String?         @map("cor_pdv")    // Cor de destaque

  criadoEm        DateTime        @default(now()) @map("criado_em")
  atualizadoEm    DateTime        @updatedAt @map("atualizado_em")

  // Relacionamentos
  categoria       CategoriaProduto? @relation(fields: [categoriaId], references: [id])
  tanques         Tanque[]
  itensVenda      ItemVenda[]
  abastecimentos  Abastecimento[]

  @@map("produtos")
}

// ============================================
// CAIXA
// ============================================

model Caixa {
  id              String        @id @default(uuid())
  postoId         String        @map("posto_id")
  numero          Int           // Número do caixa/terminal
  identificador   String?       // Nome do terminal
  operadorId      String        @map("operador_id")

  // Valores
  valorAbertura   Decimal       @map("valor_abertura") @db.Decimal(10, 2)
  valorFechamento Decimal?      @map("valor_fechamento") @db.Decimal(10, 2)
  valorEsperado   Decimal?      @map("valor_esperado") @db.Decimal(10, 2)
  diferenca       Decimal?      @db.Decimal(10, 2)

  // Status e horários
  status          StatusCaixa   @default(ABERTO)
  abertoEm        DateTime      @default(now()) @map("aberto_em")
  fechadoEm       DateTime?     @map("fechado_em")

  observacoes     String?

  criadoEm        DateTime      @default(now()) @map("criado_em")
  atualizadoEm    DateTime      @updatedAt @map("atualizado_em")

  // Relacionamentos
  posto           Posto         @relation(fields: [postoId], references: [id])
  operador        Usuario       @relation(fields: [operadorId], references: [id])
  vendas          Venda[]
  movimentos      MovimentoCaixa[]

  @@map("caixas")
}

model MovimentoCaixa {
  id              String              @id @default(uuid())
  caixaId         String              @map("caixa_id")
  tipo            TipoMovimentoCaixa
  valor           Decimal             @db.Decimal(10, 2)
  observacao      String?
  operadorId      String              @map("operador_id")

  criadoEm        DateTime            @default(now()) @map("criado_em")

  // Relacionamentos
  caixa           Caixa               @relation(fields: [caixaId], references: [id])
  operador        Usuario             @relation(fields: [operadorId], references: [id])

  @@map("movimentos_caixa")
}

// ============================================
// VENDAS
// ============================================

model Venda {
  id              String        @id @default(uuid())
  numero          Int           @default(autoincrement()) // Número sequencial
  caixaId         String        @map("caixa_id")
  operadorId      String        @map("operador_id")
  frentistaId     String?       @map("frentista_id")
  clienteId       String?       @map("cliente_id")

  // Valores
  subtotal        Decimal       @db.Decimal(10, 2)
  desconto        Decimal       @default(0) @db.Decimal(10, 2)
  acrescimo       Decimal       @default(0) @db.Decimal(10, 2)
  total           Decimal       @db.Decimal(10, 2)

  // Status
  status          StatusVenda   @default(EM_ANDAMENTO)

  // Fiscal
  cupomFiscal     String?       @map("cupom_fiscal") // Número CF-e/NFC-e
  chaveAcesso     String?       @map("chave_acesso") // Chave de acesso fiscal
  xmlFiscal       String?       @map("xml_fiscal")   // XML do documento

  // Timestamps
  iniciadaEm      DateTime      @default(now()) @map("iniciada_em")
  finalizadaEm    DateTime?     @map("finalizada_em")
  canceladaEm     DateTime?     @map("cancelada_em")

  // Sincronização offline
  sincronizado    Boolean       @default(true)
  uuidLocal       String?       @map("uuid_local") // UUID gerado offline

  criadoEm        DateTime      @default(now()) @map("criado_em")
  atualizadoEm    DateTime      @updatedAt @map("atualizado_em")

  // Relacionamentos
  caixa           Caixa         @relation(fields: [caixaId], references: [id])
  operador        Usuario       @relation("OperadorVenda", fields: [operadorId], references: [id])
  frentista       Usuario?      @relation("FrentistaVenda", fields: [frentistaId], references: [id])
  cliente         Cliente?      @relation(fields: [clienteId], references: [id])
  itens           ItemVenda[]
  pagamentos      Pagamento[]
  abastecimentos  Abastecimento[]

  @@map("vendas")
}

model ItemVenda {
  id              String        @id @default(uuid())
  vendaId         String        @map("venda_id")
  produtoId       String        @map("produto_id")
  abastecimentoId String?       @map("abastecimento_id")

  quantidade      Decimal       @db.Decimal(10, 3)
  precoUnitario   Decimal       @map("preco_unitario") @db.Decimal(10, 4)
  desconto        Decimal       @default(0) @db.Decimal(10, 2)
  acrescimo       Decimal       @default(0) @db.Decimal(10, 2)
  total           Decimal       @db.Decimal(10, 2)

  // Ordem no cupom
  sequencia       Int           @default(1)

  cancelado       Boolean       @default(false)

  criadoEm        DateTime      @default(now()) @map("criado_em")

  // Relacionamentos
  venda           Venda         @relation(fields: [vendaId], references: [id])
  produto         Produto       @relation(fields: [produtoId], references: [id])
  abastecimento   Abastecimento? @relation(fields: [abastecimentoId], references: [id])

  @@map("itens_venda")
}

// ============================================
// ABASTECIMENTOS
// ============================================

model Abastecimento {
  id              String              @id @default(uuid())
  bombaId         String              @map("bomba_id")
  bicoId          String              @map("bico_id")
  produtoId       String              @map("produto_id")
  frentistaId     String?             @map("frentista_id")
  vendaId         String?             @map("venda_id")

  // Dados do abastecimento
  litros          Decimal             @db.Decimal(10, 3)
  precoLitro      Decimal             @map("preco_litro") @db.Decimal(10, 4)
  valorTotal      Decimal             @map("valor_total") @db.Decimal(10, 2)

  // Encerrantes
  encerranteInicial Decimal           @map("encerrante_inicial") @db.Decimal(12, 3)
  encerranteFinal   Decimal           @map("encerrante_final") @db.Decimal(12, 3)

  // Placa do veículo (opcional)
  placa           String?
  hodometro       Int?

  status          StatusAbastecimento @default(PENDENTE)

  iniciadoEm      DateTime            @default(now()) @map("iniciado_em")
  finalizadoEm    DateTime?           @map("finalizado_em")

  criadoEm        DateTime            @default(now()) @map("criado_em")
  atualizadoEm    DateTime            @updatedAt @map("atualizado_em")

  // Relacionamentos
  bomba           Bomba               @relation(fields: [bombaId], references: [id])
  bico            Bico                @relation(fields: [bicoId], references: [id])
  produto         Produto             @relation(fields: [produtoId], references: [id])
  frentista       Usuario?            @relation(fields: [frentistaId], references: [id])
  venda           Venda?              @relation(fields: [vendaId], references: [id])
  itensVenda      ItemVenda[]

  @@map("abastecimentos")
}

// ============================================
// PAGAMENTOS
// ============================================

model Pagamento {
  id              String          @id @default(uuid())
  vendaId         String          @map("venda_id")
  tipo            TipoPagamento
  valor           Decimal         @db.Decimal(10, 2)
  status          StatusPagamento @default(PENDENTE)

  // Dados específicos por tipo
  troco           Decimal?        @db.Decimal(10, 2)       // Dinheiro
  nsu             String?                                   // Cartão - NSU
  autorizacao     String?                                   // Cartão - Código autorização
  bandeira        String?                                   // Cartão - Visa, Master, etc
  parcelas        Int?                                      // Cartão - Número parcelas
  txid            String?                                   // PIX - ID transação
  endToEnd        String?         @map("end_to_end")       // PIX - EndToEndId
  qrcode          String?         @db.Text                  // PIX - QR Code

  processadoEm    DateTime?       @map("processado_em")

  criadoEm        DateTime        @default(now()) @map("criado_em")
  atualizadoEm    DateTime        @updatedAt @map("atualizado_em")

  // Relacionamentos
  venda           Venda           @relation(fields: [vendaId], references: [id])

  @@map("pagamentos")
}

// ============================================
// CLIENTES
// ============================================

model Cliente {
  id              String    @id @default(uuid())
  tipo            String    @default("PF") // PF ou PJ
  nome            String
  cpfCnpj         String?   @unique @map("cpf_cnpj")

  // Contato
  telefone        String?
  email           String?

  // Endereço
  endereco        String?
  numero          String?
  complemento     String?
  bairro          String?
  cidade          String?
  estado          String?   @db.Char(2)
  cep             String?

  // Crédito/Fiado
  limiteCredito   Decimal?  @map("limite_credito") @db.Decimal(10, 2)
  saldoDevedor    Decimal   @default(0) @map("saldo_devedor") @db.Decimal(10, 2)

  // Frota
  isFrota         Boolean   @default(false) @map("is_frota")

  ativo           Boolean   @default(true)
  observacoes     String?

  criadoEm        DateTime  @default(now()) @map("criado_em")
  atualizadoEm    DateTime  @updatedAt @map("atualizado_em")

  // Relacionamentos
  vendas          Venda[]
  veiculos        Veiculo[]

  @@map("clientes")
}

model Veiculo {
  id              String    @id @default(uuid())
  clienteId       String    @map("cliente_id")
  placa           String    @unique
  modelo          String?
  cor             String?
  hodometroAtual  Int?      @map("hodometro_atual")

  ativo           Boolean   @default(true)

  criadoEm        DateTime  @default(now()) @map("criado_em")
  atualizadoEm    DateTime  @updatedAt @map("atualizado_em")

  // Relacionamentos
  cliente         Cliente   @relation(fields: [clienteId], references: [id])

  @@map("veiculos")
}

// ============================================
// AUDITORIA
// ============================================

model LogAuditoria {
  id              String    @id @default(uuid())
  usuarioId       String?   @map("usuario_id")
  acao            String    // CREATE, UPDATE, DELETE, LOGIN, etc
  entidade        String    // Nome da tabela/entidade
  entidadeId      String?   @map("entidade_id")
  dadosAntigos    Json?     @map("dados_antigos")
  dadosNovos      Json?     @map("dados_novos")
  ip              String?
  userAgent       String?   @map("user_agent")

  criadoEm        DateTime  @default(now()) @map("criado_em")

  // Relacionamentos
  usuario         Usuario?  @relation(fields: [usuarioId], references: [id])

  @@index([entidade, entidadeId])
  @@index([criadoEm])
  @@map("logs_auditoria")
}

// ============================================
// CONFIGURAÇÕES DO SISTEMA
// ============================================

model ConfiguracaoSistema {
  id              String    @id @default(uuid())
  chave           String    @unique
  valor           String
  tipo            String    @default("string") // string, number, boolean, json
  descricao       String?
  grupo           String?   // Agrupamento: fiscal, impressao, pdv, etc

  criadoEm        DateTime  @default(now()) @map("criado_em")
  atualizadoEm    DateTime  @updatedAt @map("atualizado_em")

  @@map("configuracoes_sistema")
}
```

---

## Atalhos de Teclado do PDV

### Atalhos Globais (Funcionam em qualquer tela)
| Tecla | Ação | Descrição |
|-------|------|-----------|
| `F1` | Ajuda | Abre painel de ajuda com atalhos |
| `F2` | Nova Venda | Inicia uma nova venda |
| `F3` | Buscar Produto | Abre busca de produtos |
| `F4` | Desconto | Aplica desconto no item/venda |
| `F5` | Sincronizar | Força sincronização com servidor |
| `F6` | Consulta Preço | Consulta preço sem vender |
| `F7` | Formas Pagamento | Abre seleção de pagamento |
| `F8` | Finalizar Venda | Conclui a venda atual |
| `F9` | Cancelar Item | Cancela último item |
| `F10` | Menu | Abre menu de opções |
| `F11` | Fullscreen | Alterna modo tela cheia |
| `F12` | Dev Tools | (Apenas em desenvolvimento) |
| `ESC` | Cancelar | Cancela operação atual |
| `ENTER` | Confirmar | Confirma ação/campo |

### Atalhos Numéricos (Bombas)
| Tecla | Ação |
|-------|------|
| `1` - `9` | Seleciona bomba 1-9 |
| `0` | Seleciona bomba 10 |
| `Alt + 1-9` | Reabre abastecimento atrasado |

### Atalhos de Pagamento (Modal F7)
| Tecla | Forma de Pagamento |
|-------|-------------------|
| `1` ou `D` | Dinheiro |
| `2` ou `B` | Débito |
| `3` ou `C` | Crédito |
| `4` ou `P` | PIX |
| `5` ou `V` | Voucher/Frota |
| `6` ou `M` | Misto (múltiplas formas) |

### Atalhos de Combustível
| Tecla | Combustível |
|-------|-------------|
| `G` | Gasolina Comum |
| `A` | Gasolina Aditivada |
| `E` | Etanol |
| `S` | Diesel S10 |
| `K` | Diesel S500 |

### Navegação
| Tecla | Ação |
|-------|------|
| `TAB` | Próximo campo |
| `SHIFT + TAB` | Campo anterior |
| `↑` / `↓` | Navegar em listas |
| `CTRL + ↑` | Aumentar quantidade |
| `CTRL + ↓` | Diminuir quantidade |

### Operações de Caixa
| Tecla | Ação |
|-------|------|
| `CTRL + A` | Abertura de caixa |
| `CTRL + F` | Fechamento de caixa |
| `CTRL + S` | Sangria |
| `CTRL + U` | Suprimento |
| `CTRL + R` | Reimprimir cupom |
| `CTRL + X` | Cancelar venda |

---

## Fluxo de Venda

### Fluxo Básico (Pista)
```
1. [F2] Iniciar Venda
   └─> Campo "Bomba" recebe foco

2. [1-9] Selecionar Bomba
   └─> Busca abastecimento pendente
   └─> Se encontrar: carrega automaticamente
   └─> Se não: aguarda seleção manual

3. [ENTER] Confirmar Abastecimento
   └─> Item adicionado ao carrinho
   └─> Pode adicionar produtos de loja (F3)

4. [F7] Selecionar Pagamento
   └─> Modal de formas de pagamento
   └─> [1] Dinheiro: solicita valor recebido
   └─> [2-3] Cartão: integra com TEF
   └─> [4] PIX: gera QR Code

5. [F8] Finalizar Venda
   └─> Emite CF-e/NFC-e
   └─> Imprime cupom
   └─> Limpa tela para próxima venda
```

### Fluxo Conveniência (Loja)
```
1. [F2] Iniciar Venda

2. [F3] Buscar Produto
   └─> Digita código ou nome
   └─> [ENTER] adiciona ao carrinho
   └─> Repete para mais produtos

3. [F7] → [F8] Pagamento e Finalização
```

### Fluxo Misto (Pista + Loja)
```
1. [F2] → [Bomba] → Abastecimento
2. [F3] → Produtos loja
3. [F7] → [F8] Finalização
```

---

## Interface do PDV

### Layout Principal (Tela de Vendas)
```
┌─────────────────────────────────────────────────────────────────────────┐
│ HEADER                                                                   │
│ ┌───────────────┬───────────────────────────────────┬─────────────────┐ │
│ │ I9 SMART PDV  │ Operador: João Silva              │ Caixa: 01      │ │
│ │ Posto XYZ     │ Turno: Manhã                      │ 14:35:22       │ │
│ └───────────────┴───────────────────────────────────┴─────────────────┘ │
├─────────────────────────────────────────────────────────────────────────┤
│ ÁREA PRINCIPAL                                                           │
│ ┌──────────────────────────────────┬────────────────────────────────────┤
│ │ BOMBAS                           │ ITENS DA VENDA                      │
│ │ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ │ ┌──────────────────────────────────┐│
│ │ │  1  │ │  2  │ │  3  │ │  4  │ │ │ 1x Gasolina Comum    45,000L     ││
│ │ │LIVRE│ │ ██  │ │LIVRE│ │ ██  │ │ │    R$ 5,89/L         R$ 265,05  ││
│ │ └─────┘ └─────┘ └─────┘ └─────┘ │ ├──────────────────────────────────┤│
│ │ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ │ │ 2x Água 500ml                    ││
│ │ │  5  │ │  6  │ │  7  │ │  8  │ │ │    R$ 3,50           R$ 7,00    ││
│ │ │LIVRE│ │LIVRE│ │LIVRE│ │LIVRE│ │ └──────────────────────────────────┘│
│ │ └─────┘ └─────┘ └─────┘ └─────┘ │                                      │
│ │                                  │──────────────────────────────────────│
│ │ ABASTECIMENTO SELECIONADO       │ TOTAIS                               │
│ │ ┌────────────────────────────┐  │ Subtotal:              R$ 272,05    │
│ │ │ Bomba: 2                   │  │ Desconto:              R$   0,00    │
│ │ │ Combustível: Gasolina      │  │ ─────────────────────────────────   │
│ │ │ Litros: 45,000             │  │ TOTAL:                 R$ 272,05    │
│ │ │ Valor: R$ 265,05           │  │                                      │
│ │ │ Frentista: Carlos          │  │                                      │
│ │ └────────────────────────────┘  │                                      │
│ └──────────────────────────────────┴────────────────────────────────────┤
├─────────────────────────────────────────────────────────────────────────┤
│ BARRA DE ATALHOS                                                         │
│ [F2] Nova │ [F3] Produto │ [F4] Desconto │ [F7] Pagto │ [F8] Finalizar │
└─────────────────────────────────────────────────────────────────────────┘
```

### Estados Visuais das Bombas
```
┌─────────────────────────────────────────────────────────────────┐
│ LEGENDA DE CORES                                                 │
├─────────────────────────────────────────────────────────────────┤
│ 🟢 VERDE      - Livre (pronta para abastecer)                   │
│ 🟡 AMARELO    - Em abastecimento (bomba ativa)                  │
│ 🔵 AZUL       - Abastecimento pendente (aguarda PDV)            │
│ 🔴 VERMELHO   - Bloqueada/Manutenção                            │
│ ⚫ CINZA      - Offline (sem comunicação)                        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Componentes UI Planejados

### Componentes Base (shadcn/ui)
1. **Button** - Com variantes e estados de loading
2. **Input** - Com suporte a máscaras
3. **Card** - Cards informativos
4. **Modal/Dialog** - Modais de ação
5. **Toast** - Notificações
6. **Badge** - Status e tags
7. **Kbd** - Indicador de tecla de atalho
8. **Spinner** - Loading states

### Componentes Específicos PDV
1. **BombaCard** - Card visual da bomba com status
2. **AbastecimentoCard** - Detalhes do abastecimento
3. **ItemVendaRow** - Linha do item na venda
4. **TotalDisplay** - Display grande de totais
5. **PaymentModal** - Modal de formas de pagamento
6. **ShortcutBar** - Barra inferior de atalhos
7. **NumPad** - Teclado numérico virtual (opcional)
8. **ProdutoSearch** - Busca rápida de produtos
9. **CaixaStatus** - Indicador de status do caixa
10. **FrentistaBadge** - Badge do frentista

---

## Próximos Passos

### Fase 1 - MVP (2-4 semanas)
- [ ] Setup do projeto (Next.js + Express + Prisma)
- [ ] Autenticação básica (JWT + PIN)
- [ ] Cadastro de produtos
- [ ] Abertura/Fechamento de caixa
- [ ] Venda simples (sem abastecimento)
- [ ] Pagamento em dinheiro
- [ ] Impressão de cupom (não fiscal)

### Fase 2 - Pista (2-4 semanas)
- [ ] Cadastro de bombas e tanques
- [ ] Simulação de abastecimentos
- [ ] Integração com PDV
- [ ] Frentistas e comissões
- [ ] Pagamentos PIX e cartão (mock)

### Fase 3 - Fiscal (4-6 semanas)
- [ ] Integração SAT/NFC-e
- [ ] Integração TEF
- [ ] PIX real (banco)
- [ ] Relatórios fiscais

### Fase 4 - Automação (4-8 semanas)
- [ ] Integração com concentradores
- [ ] Leitura automática de abastecimentos
- [ ] LMC (Livro de Movimentação de Combustíveis)
- [ ] Integração com tanques

---

## Considerações Finais

Este documento serve como guia completo para o desenvolvimento do I9 Smart PDV Web. A estrutura foi projetada para:

1. **Performance** - Foco em teclado para operação rápida
2. **Offline-first** - Funciona mesmo sem internet
3. **Escalabilidade** - Arquitetura modular e extensível
4. **Compliance** - Preparado para requisitos fiscais brasileiros
5. **UX** - Interface limpa e intuitiva para operadores

O sistema segue os mesmos padrões do projeto ECTM, facilitando manutenção e conhecimento compartilhado entre projetos.
