# I9 Smart PDV Web - Módulo Tributário

Documentação do Configurador Tributário inspirado no TOTVS Protheus.
Sistema completo para gestão fiscal de postos de combustíveis.

---

## 1. Visão Geral

O módulo tributário foi projetado para:
- Automatizar cálculos fiscais conforme legislação brasileira
- Suportar tributação monofásica de combustíveis (NT 2023.001)
- Atender a Lei da Transparência Fiscal (Lei 12.741/2012)
- Simplificar configuração tributária por produto/NCM

### Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    MÓDULO TRIBUTÁRIO                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Tabelas    │  │   Config.    │  │   Cálculo    │       │
│  │  Referência  │  │  Tributária  │  │   Tributos   │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│         │                 │                 │               │
│         ▼                 ▼                 ▼               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                   PRISMA / PostgreSQL                │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Tabelas de Referência

### 2.1 Resumo das Tabelas

| Tabela | Registros | Fonte | Descrição |
|--------|-----------|-------|-----------|
| `TribTributo` | 6 | Estático | Tributos base (ICMS, PIS, COFINS, IPI, ISS, FCP) |
| `TribCST` | 92 | Estático | Códigos de Situação Tributária |
| `TribCSOSN` | 10 | Estático | Códigos Simples Nacional |
| `TribCFOP` | 247 | Estático | Códigos Fiscais de Operações |
| `TribANP` | 22 | Estático | Códigos ANP de Combustíveis |
| `TribAliquotaUF` | 27 | Estático | Alíquotas ICMS por UF |
| `TribNCM` | 10.507 | Siscomex | Nomenclatura Comum Mercosul |
| `Uf` | 27 | IBGE | Estados Brasileiros |
| `Municipio` | 5.570 | IBGE | Municípios Brasileiros |
| `TribIBPT` | 98+ | CDN nfe.io | Alíquotas Lei Transparência |

### 2.2 Detalhamento

#### TribTributo
Tributos base do sistema fiscal brasileiro.

| Código | Descrição | Esfera | Espécie |
|--------|-----------|--------|---------|
| ICMS | Imposto sobre Circulação de Mercadorias | ESTADUAL | IMPOSTO |
| PIS | Programa de Integração Social | FEDERAL | CONTRIBUICAO |
| COFINS | Contribuição para Financiamento da Seguridade | FEDERAL | CONTRIBUICAO |
| IPI | Imposto sobre Produtos Industrializados | FEDERAL | IMPOSTO |
| ISS | Imposto sobre Serviços | MUNICIPAL | IMPOSTO |
| FCP | Fundo de Combate à Pobreza | ESTADUAL | ADICIONAL |

#### TribCST (Código de Situação Tributária)

**ICMS (12 códigos)**
| Código | Descrição | Gera Tributo |
|--------|-----------|--------------|
| 00 | Tributada integralmente | Sim |
| 10 | Tributada com ST | Sim |
| 20 | Com redução de base | Sim |
| 30 | Isenta/não tributada com ST | Não |
| 40 | Isenta | Não |
| 41 | Não tributada | Não |
| 50 | Suspensão | Não |
| 51 | Diferimento | Não |
| 60 | ICMS cobrado anteriormente por ST | Não |
| 61 | Tributação monofásica combustíveis | Não |
| 70 | Redução de base + ST | Sim |
| 90 | Outras | Depende |

**PIS/COFINS (33 códigos cada)**
| Código | Descrição |
|--------|-----------|
| 01 | Alíquota básica (1,65% / 7,6%) |
| 02 | Alíquota diferenciada |
| 03 | Alíquota por unidade |
| 04 | Monofásica - revenda vedada |
| 05 | Monofásica - revenda ST |
| 06 | Alíquota zero |
| 07 | Isenta |
| 08 | Sem incidência |
| 09 | Suspensão |
| ... | (demais códigos) |

#### TribCSOSN (Simples Nacional)
| Código | Descrição | Permite Crédito |
|--------|-----------|-----------------|
| 101 | Tributada com permissão de crédito | Sim |
| 102 | Tributada sem permissão de crédito | Não |
| 103 | Isenção do ICMS no Simples | Não |
| 201 | Tributada com crédito e ST | Sim |
| 202 | Tributada sem crédito e com ST | Não |
| 203 | Isenção com ST | Não |
| 300 | Imune | Não |
| 400 | Não tributada | Não |
| 500 | ICMS cobrado anteriormente por ST | Não |
| 900 | Outros | Depende |

#### TribANP (Combustíveis)
Códigos da Agência Nacional do Petróleo com tributação monofásica (Ad Rem).

| Código | Descrição | Alíquota Ad Rem |
|--------|-----------|-----------------|
| 210203001 | Gasolina A | R$ 1,2571/L |
| 210203003 | Gasolina C | R$ 1,2571/L |
| 420101001 | Diesel S10 | R$ 0,9456/L |
| 420101002 | Diesel S500 | R$ 0,9456/L |
| 420201001 | Biodiesel B100 | R$ 0,9456/L |
| 220102002 | Etanol Hidratado | R$ 0,1279/L |
| 220102003 | Etanol Anidro | R$ 0,0000/L |
| 610101001 | GLP a granel | R$ 1,2571/kg |

---

## 3. Fontes de Dados Oficiais

### 3.1 Siscomex - NCM
- **URL**: `https://portalunico.siscomex.gov.br/classif/api/publico/nomenclatura/download/json`
- **Dados**: ~10.500 códigos NCM vigentes
- **Atualização**: Mensal

### 3.2 IBGE - Localidades
- **Estados**: `https://servicodados.ibge.gov.br/api/v1/localidades/estados`
- **Municípios**: `https://servicodados.ibge.gov.br/api/v1/localidades/municipios`
- **Dados**: 27 UFs + 5.570 municípios

### 3.3 IBPT - Lei Transparência
- **URL**: `http://ibpt.nfe.io/ncm/{uf}/{ncm}.json`
- **Dados**: Alíquotas aproximadas por NCM
- **Base Legal**: Lei 12.741/2012

---

## 4. Scripts de Seed

### 4.1 Localização
```
backend/src/prisma/seeds/tributos/
├── index.ts              # Seed tabelas estáticas
├── seed-all.ts           # Orquestrador principal
├── ncm-downloader.ts     # Download NCM Siscomex
├── cfop-downloader.ts    # CFOP completo
├── ibge-downloader.ts    # Estados e Municípios
├── ibpt-downloader.ts    # Lei Transparência
├── cache/                # Cache de downloads (30 dias)
│   ├── ncm-siscomex.json
│   ├── ibge-estados.json
│   ├── ibge-municipios.json
│   └── ibpt-ncms.json
└── data/                 # Dados estáticos
    ├── tributos.ts
    ├── cst.ts
    ├── csosn.ts
    ├── cfop.ts
    ├── anp.ts
    └── aliquotas-uf.ts
```

### 4.2 Comandos

```bash
# Seed completo (todas as tabelas)
npx tsx src/prisma/seeds/tributos/seed-all.ts --full

# Forçar novo download (ignorar cache)
npx tsx src/prisma/seeds/tributos/seed-all.ts --full --force

# Apenas tabelas específicas
npx tsx src/prisma/seeds/tributos/seed-all.ts --ncm    # NCM do Siscomex
npx tsx src/prisma/seeds/tributos/seed-all.ts --cfop   # CFOP completo
npx tsx src/prisma/seeds/tributos/seed-all.ts --ibge   # Estados/Municípios
npx tsx src/prisma/seeds/tributos/seed-all.ts --ibpt   # Lei Transparência

# Seed básico (sem downloads)
npx tsx src/prisma/seeds/tributos/seed-all.ts
```

### 4.3 Cache
Os downloads são cacheados por 30 dias em `cache/`. Use `--force` para ignorar.

---

## 5. Schema Prisma

### 5.1 Enums

```prisma
enum TribEsfera {
  FEDERAL
  ESTADUAL
  MUNICIPAL
}

enum TribEspecie {
  IMPOSTO
  TAXA
  CONTRIBUICAO
  ADICIONAL
}

enum TribTipoCST {
  ICMS
  PIS
  COFINS
  IPI
}

enum TribTipoOperacao {
  VENDA_INTERNA
  VENDA_INTERESTADUAL
  VENDA_EXTERIOR
  COMPRA_INTERNA
  COMPRA_INTERESTADUAL
  COMPRA_EXTERIOR
  DEVOLUCAO
  TRANSFERENCIA
  REMESSA
  RETORNO
  PRESTACAO_SERVICO
  OUTROS
}

enum TribFinalidade {
  NORMAL
  COMPLEMENTAR
  DEVOLUCAO
  AJUSTE
}
```

### 5.2 Tabelas Principais

```prisma
model TribTributo {
  id        String       @id @default(uuid())
  codigo    String       @unique
  descricao String
  esfera    TribEsfera
  especie   TribEspecie
  ativo     Boolean      @default(true)
  createdAt DateTime     @default(now())
  updatedAt DateTime     @updatedAt
}

model TribNCM {
  id           String   @id @default(uuid())
  codigo       String   @unique // 8 dígitos
  descricao    String
  unidadePadrao String?
  ativo        Boolean  @default(true)
  createdAt    DateTime @default(now())
  updatedAt    DateTime @updatedAt
}

model TribANP {
  id           String   @id @default(uuid())
  codigo       String   @unique // 9 dígitos
  descricao    String
  unidade      String   // L, KG, M3
  ncmPadrao    String?
  isMonofasico Boolean  @default(true)
  aliquotaAdRem Decimal? @db.Decimal(10, 4)
  ativo        Boolean  @default(true)
  createdAt    DateTime @default(now())
  updatedAt    DateTime @updatedAt
}

model TribIBPT {
  id                  String    @id @default(uuid())
  ncm                 String    @unique
  descricao           String
  aliqFederalNacional Decimal   @db.Decimal(6, 2)
  aliqFederalImportado Decimal  @db.Decimal(6, 2)
  aliqEstadual        Decimal   @db.Decimal(6, 2)
  aliqMunicipal       Decimal   @db.Decimal(6, 2)
  versao              String
  fonte               String
  vigenciaInicio      DateTime
  vigenciaFim         DateTime?
  createdAt           DateTime  @default(now())
  updatedAt           DateTime  @updatedAt
}

model Uf {
  id          String      @id @default(uuid())
  codigoIbge  Int         @unique
  sigla       String      @unique @db.Char(2)
  nome        String
  regiao      String
  createdAt   DateTime    @default(now())
  updatedAt   DateTime    @updatedAt
  municipios  Municipio[]
}

model Municipio {
  id         String   @id @default(uuid())
  codigoIbge Int      @unique
  nome       String
  ufSigla    String   @db.Char(2)
  uf         Uf       @relation(fields: [ufSigla], references: [sigla])
  createdAt  DateTime @default(now())
  updatedAt  DateTime @updatedAt
}
```

---

## 6. Tributação Monofásica (Combustíveis)

### 6.1 Conceito
A NT 2023.001 implementou a tributação monofásica (Ad Rem) para combustíveis, substituindo o ICMS por alíquota percentual por alíquota fixa por litro/kg.

### 6.2 Alíquotas Ad Rem (vigentes)

| Combustível | Alíquota | Unidade |
|-------------|----------|---------|
| Gasolina | R$ 1,2571 | por litro |
| Diesel | R$ 0,9456 | por litro |
| GLP | R$ 1,2571 | por kg |
| Etanol Hidratado | R$ 0,1279 | por litro |
| Etanol Anidro | R$ 0,0000 | por litro |

### 6.3 CST para Monofásico
- **CST ICMS**: 61 (Tributação monofásica sobre combustíveis)
- **CST PIS/COFINS**: 04 (Monofásica - Revenda tributada)

### 6.4 CFOP para Combustíveis
| CFOP | Operação |
|------|----------|
| 5.656 | Venda combustível a consumidor final |
| 5.667 | Venda combustível para não contribuinte |
| 6.656 | Venda interestadual combustível |

---

## 7. Lei da Transparência Fiscal (Lei 12.741/2012)

### 7.1 Obrigatoriedade
Todos os documentos fiscais devem informar o valor aproximado dos tributos incidentes sobre cada produto.

### 7.2 Tributos Informados
| Tributo | Descrição |
|---------|-----------|
| Federal Nacional | PIS + COFINS + IPI (produtos nacionais) |
| Federal Importado | PIS + COFINS + IPI + II (produtos importados) |
| Estadual | ICMS |
| Municipal | ISS (serviços) |

### 7.3 Cálculo
```javascript
// Exemplo de cálculo para produto nacional
const valorProduto = 100.00;
const ibpt = await prisma.tribIBPT.findUnique({ where: { ncm: '27101259' } });

const tributoFederal = valorProduto * (ibpt.aliqFederalNacional / 100);
const tributoEstadual = valorProduto * (ibpt.aliqEstadual / 100);
const tributoMunicipal = valorProduto * (ibpt.aliqMunicipal / 100);
const tributoTotal = tributoFederal + tributoEstadual + tributoMunicipal;

// Exibir no cupom fiscal
// "Valor aproximado dos tributos: R$ XX,XX (XX,XX%)"
```

---

## 8. Configuração Tributária por Produto

### 8.1 Modelo TribConfigProduto
```prisma
model TribConfigProduto {
  id              String   @id @default(uuid())
  empresaId       String
  produtoId       String
  regimeTributario TribRegime

  // ICMS
  cstIcms         String
  aliqIcms        Decimal?
  reducaoBaseIcms Decimal?

  // PIS
  cstPis          String
  aliqPis         Decimal?

  // COFINS
  cstCofins       String
  aliqCofins      Decimal?

  // CFOP
  cfopVendaInterna      String?
  cfopVendaInterestadual String?

  // Monofásico
  isMonofasico    Boolean  @default(false)
  aliquotaAdRem   Decimal?

  vigenciaInicio  DateTime
  vigenciaFim     DateTime?
  ativo           Boolean  @default(true)
}
```

### 8.2 Hierarquia de Configuração
1. Produto específico (TribConfigProduto)
2. NCM do produto (TribNCM + TribIBPT)
3. Configuração padrão da empresa

---

## 9. Próximos Passos

### 9.1 Implementados
- [x] Schema Prisma com todas as tabelas tributárias
- [x] Seeds de dados estáticos (CST, CSOSN, CFOP, ANP)
- [x] Download automático NCM do Siscomex
- [x] Download automático IBGE (UF + Municípios)
- [x] Download automático IBPT (Lei Transparência)
- [x] Sistema de cache com 30 dias de validade

### 9.2 Pendentes
- [ ] API REST para consulta de tabelas tributárias
- [ ] Tela de configuração tributária por produto
- [ ] Cálculo automático de tributos na venda
- [ ] Geração de NF-e/NFC-e com tributação correta
- [ ] Integração com IBPT via API (atualização automática)
- [ ] Relatórios fiscais (Livros de Entrada/Saída)

---

## 10. Referências

- [Portal Único Siscomex](https://portalunico.siscomex.gov.br/)
- [IBGE Localidades](https://servicodados.ibge.gov.br/api/docs/localidades)
- [IBPT - De Olho no Imposto](https://deolhonoimposto.ibpt.org.br/)
- [NT 2023.001 - Tributação Monofásica](https://www.nfe.fazenda.gov.br/)
- [Manual NF-e](https://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=33ol5hhSYZk=)

---

*Documento gerado em: Dezembro/2024*
*Versão: 1.0*
