# I9 Smart PDV Web - Instruções para Claude

## Contexto do Projeto

Sistema de Ponto de Venda (PDV) para **Postos de Combustíveis** com arquitetura multi-empresa. O projeto é dividido em três aplicações:

- **backend/** - API REST com Express + Prisma + PostgreSQL
- **frontend/** - Aplicação web com Next.js 16 + React 19
- **mobile/** - App mobile com React Native + Expo

## Padrões de Código

### Nomenclatura
- **Banco de dados**: snake_case em português (ex: `grupos_empresa`, `tipo_unidade`)
- **TypeScript**: camelCase (ex: `grupoEmpresa`, `tipoUnidade`)
- **Componentes React**: PascalCase (ex: `BombaCard`, `VendaPanel`)
- **Arquivos**: kebab-case ou camelCase conforme contexto

### Estrutura de Arquivos

```
backend/
├── src/
│   ├── controllers/    # Controladores de rotas
│   ├── services/       # Lógica de negócio
│   ├── routes/         # Definição de rotas
│   ├── middlewares/    # Middlewares Express
│   ├── schemas/        # Schemas Zod
│   └── prisma/         # Schema e migrations
```

```
frontend/
├── src/
│   ├── app/           # Next.js App Router
│   ├── components/    # Componentes React
│   ├── stores/        # Zustand stores
│   ├── hooks/         # Custom hooks
│   └── lib/           # Utilitários
```

## Regras de Negócio Importantes

### Hierarquia Organizacional
1. **GrupoEmpresa** → agrupa várias empresas
2. **Empresa** → posto individual (MATRIZ ou FILIAL)
3. **Estacao** → terminal físico com periféricos
4. **Usuario** → perfil único, pode trabalhar em qualquer estação

### Perfis de Usuário
| Perfil | Escopo |
|--------|--------|
| SUPER_ADMIN | Todo o sistema |
| ADMIN_GRUPO | Todas empresas do grupo |
| GERENTE_GERAL | Visualiza todas empresas do grupo |
| GERENTE_UNIDADE | Uma empresa específica |
| OPERADOR | Apenas suas vendas/caixa |
| FRENTISTA | Apenas abastecimentos |

### Caixa (Controle Financeiro)
- Vinculado ao **usuário** (não à estação)
- Representa o controle de movimentação financeira do turno
- Usuário pode trocar de estação mantendo o mesmo caixa

### Descontos Automáticos
| Combustível | Pagamento | Desconto |
|-------------|-----------|----------|
| Diesel | Dinheiro/PIX | R$ 0,35/L |
| Diesel | Débito | R$ 0,25/L |
| Diesel | Crédito | R$ 0,00 |

## Comandos Úteis

### Backend
```bash
cd backend
npm run dev              # Desenvolvimento
npm run prisma:generate  # Gerar Prisma Client
npm run prisma:push      # Push schema para BD
npm run prisma:studio    # Abrir Prisma Studio
npm run test             # Executar testes
```

### Frontend
```bash
cd frontend
npm run dev    # Desenvolvimento
npm run build  # Build produção
npm run lint   # Linting
```

### Mobile
```bash
cd mobile
npx expo start           # Desenvolvimento
npx expo start --android # Android
npx expo start --ios     # iOS
```

## Atalhos do PDV (Keyboard-First)

| Tecla | Ação |
|-------|------|
| F2 | Nova venda |
| F3 | Buscar produto |
| F7 | Formas de pagamento |
| F8 | Finalizar venda |
| 1-9 | Selecionar bomba |
| ESC | Cancelar |

## Documentação Adicional

- [PROJETO.md](./PROJETO.md) - Arquitetura completa
- [docs/FUNCIONALIDADES.md](./docs/FUNCIONALIDADES.md) - Funcionalidades do sistema
- [docs/MULTI-EMPRESA.md](./docs/MULTI-EMPRESA.md) - Estrutura multi-empresa
- [backend/src/prisma/schema.prisma](./backend/src/prisma/schema.prisma) - Schema do banco

## Ao Desenvolver

1. **Sempre validar inputs** com Zod
2. **Filtrar dados por contexto** (empresa, grupo) automaticamente
3. **Usar transações** para operações que afetam múltiplas tabelas
4. **Registrar logs de auditoria** para operações críticas
5. **Testar offline** - o sistema deve funcionar sem internet
6. **Priorizar teclado** - todas operações devem ser possíveis via teclado
