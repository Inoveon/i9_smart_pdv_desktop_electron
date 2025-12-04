# I9 Smart PDV Web

Sistema de Ponto de Venda (PDV) completo para **Postos de Combustíveis**.

## Características

- **Teclado-First**: Operação 100% via teclado para máxima velocidade
- **Offline-First**: Funciona mesmo sem internet, sincroniza automaticamente
- **Multi-plataforma**: Web (desktop), Mobile (Android/iOS)
- **Fiscal**: Preparado para SAT, NFC-e, CF-e
- **Integração**: Bombas, TEF, PIX, impressoras térmicas

## Estrutura do Projeto

```
i9_smart_pdv_web/
├── frontend/          # Next.js 16 + React 19 (Web)
├── backend/           # Express + Prisma + PostgreSQL
├── mobile/            # React Native + Expo (Android/iOS)
├── docs/              # Documentação adicional
├── PROJETO.md         # Documentação técnica completa
└── README.md          # Este arquivo
```

## Stack Tecnológico

| Camada | Tecnologias |
|--------|-------------|
| Frontend Web | Next.js 16, React 19, TypeScript, Tailwind, shadcn/ui |
| Backend | Node.js, Express, Prisma, PostgreSQL |
| Mobile | React Native, Expo, NativeWind |

## Pré-requisitos

- Node.js 18+
- PostgreSQL 15+
- pnpm ou npm

## Instalação

```bash
# Clone o repositório
git clone https://github.com/leechardes/i9_smart_pdv_web.git

# Backend
cd backend
npm install
npm run prisma:generate
npm run prisma:push
npm run dev

# Frontend (outro terminal)
cd frontend
npm install
npm run dev

# Mobile (outro terminal)
cd mobile
npm install
npx expo start
```

## Atalhos do PDV

| Tecla | Ação |
|-------|------|
| F2 | Nova venda |
| F3 | Buscar produto |
| F7 | Formas de pagamento |
| F8 | Finalizar venda |
| 1-9 | Selecionar bomba |
| ESC | Cancelar |

## Arquitetura Multi-Empresa

O sistema suporta múltiplos postos organizados em:

```
Grupo de Empresas (Rede/Holding)
├── Empresa (Matriz)
│   ├── Estações (PDV-01, PDV-02...)
│   └── Filiais vinculadas
└── Usuários com perfis e permissões
```

## Documentação

- [PROJETO.md](./PROJETO.md) - Documentação técnica completa
- [CLAUDE.md](./CLAUDE.md) - Instruções para desenvolvimento com IA
- [docs/FUNCIONALIDADES.md](./docs/FUNCIONALIDADES.md) - Funcionalidades do sistema
- [docs/MULTI-EMPRESA.md](./docs/MULTI-EMPRESA.md) - Estrutura multi-empresa
- [Schema Prisma](./backend/src/prisma/schema.prisma) - Modelo de dados

## Licença

Proprietary - I9 Smart
