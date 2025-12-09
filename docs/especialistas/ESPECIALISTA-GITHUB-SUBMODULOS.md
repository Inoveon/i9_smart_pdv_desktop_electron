# 🔗 Especialista em GitHub & Submodules

## Visão Geral

Este documento descreve o especialista responsável por gerenciar repositórios GitHub, commits, branches e submodules no projeto i9 Smart PDV Web.

## 📋 Responsabilidades

### 1. Estrutura de Repositórios

O projeto i9 Smart PDV Web é um **monorepo com submodules**, organizados da seguinte forma:

```
i9_smart_pdv_web/
├── backend/          → Submodule: i9_smart_pdv_api_express
├── frontend/         → Submodule: i9_smart_pdv_web_nextjs
├── mobile/           → Submodule: i9_smart_pdv_mobile_expo
├── desktop/          → Submodule: i9_smart_pdv_desktop_electron (futuro)
├── .gitmodules       → Configuração dos submodules
└── docs/
    └── especialistas/
        └── ESPECIALISTA-GITHUB-SUBMODULOS.md (este arquivo)
```

### 2. Padrão de Nomenclatura de Repositórios

**Formato:** `i9_smart_pdv_{tipo}_{tecnologia}`

**Tipos Disponíveis:**
- `api` → Express, Node.js (backend)
- `web` → Next.js, React (frontend web)
- `mobile` → Expo, React Native (mobile)
- `desktop` → Electron, React (desktop/PDV)

**Exemplos:**
```
i9_smart_pdv_api_express         (backend)
i9_smart_pdv_web_nextjs          (frontend web)
i9_smart_pdv_mobile_expo         (mobile)
i9_smart_pdv_desktop_electron    (desktop - novo)
```

## 🔧 Gerenciamento de Submodules

### Configuração Atual (.gitmodules)

```ini
[submodule "backend"]
	path = backend
	url = https://github.com/leechardes/i9_smart_pdv_api_express.git

[submodule "frontend"]
	path = frontend
	url = https://github.com/leechardes/i9_smart_pdv_web_nextjs.git

[submodule "mobile"]
	path = mobile
	url = https://github.com/leechardes/i9_smart_pdv_mobile_expo.git

[submodule "desktop"]
	path = desktop
	url = https://github.com/leechardes/i9_smart_pdv_desktop_electron.git
```

### Operações com Submodules

#### 1. Clonar Repositório com Submodules

```bash
# Clone recursivo (recomendado)
git clone --recurse-submodules https://github.com/leechardes/i9_smart_pdv_web.git

# Ou clone normal + atualizar submodules
git clone https://github.com/leechardes/i9_smart_pdv_web.git
cd i9_smart_pdv_web
git submodule update --init --recursive
```

#### 2. Atualizar Submodules para Última Versão

```bash
# Atualizar todos os submodules
git submodule update --remote

# Atualizar submodule específico
git submodule update --remote frontend

# Com merge automático
git submodule update --remote --merge
```

#### 3. Adicionar Novo Submodule

```bash
# Adicionar submodule
git submodule add https://github.com/leechardes/i9_smart_pdv_desktop_electron.git desktop

# Commit das mudanças
git add .gitmodules desktop
git commit -m "chore: adiciona submodule desktop"
```

#### 4. Remover Submodule

```bash
# Remover do índice
git rm --cached desktop

# Remover do .gitmodules
git config --file=.gitmodules --remove-section submodule.desktop
git add .gitmodules

# Remover pasta
rm -rf desktop

# Commit
git commit -m "chore: remove submodule desktop"
```

#### 5. Atualizar Referência do Submodule (Mais Comum)

Quando há commits novos em um submodule (ex: frontend), você precisa atualizar a referência:

```bash
cd frontend
git pull origin main
cd ..

# Ou direto no parent
git add frontend
git commit -m "chore: atualiza referência do submodule frontend com [descrição da mudança]"
git push origin main
```

## 📝 Padrão de Commits

### Formato de Commit

```
<tipo>(<escopo>): <descrição>

[corpo opcional]

[rodapé opcional]
```

### Tipos de Commit

| Tipo | Descrição | Exemplo |
|------|-----------|---------|
| `feat` | Nova funcionalidade | `feat: adiciona autenticação OAuth` |
| `fix` | Correção de bug | `fix: corrige crash ao fazer login` |
| `chore` | Tarefas de manutenção | `chore: atualiza dependências` |
| `docs` | Documentação | `docs: atualiza README` |
| `style` | Formatação de código | `style: remove espaços desnecessários` |
| `refactor` | Refatoração sem mudança de funcionalidade | `refactor: melhora estrutura de pastas` |
| `test` | Adição/alteração de testes | `test: adiciona testes de autenticação` |
| `perf` | Melhoria de performance | `perf: otimiza query de banco de dados` |

### Padrão para Atualizações de Submodules

```bash
# Formato padrão
chore: atualiza referência do submodule {nome} com {descrição}

# Exemplos válidos
chore: atualiza referência do submodule frontend com correções de Select
chore: atualiza referência do submodule backend com novas rotas de fiscal
chore: atualiza referência do submodule mobile com suporte a dark mode
```

### Commits que NÃO Devem Ter

❌ **NÃO adicionar:**
- `Co-Authored-By: Claude <noreply@anthropic.com>` (comentário do Claude)
- Comentários como `# TODO` ou `# FIXME` em commits
- Mensagens genéricas como "update" ou "fix"

✅ **SEMPRE:**
- Usar verbos no imperativo: "adiciona", "corrige", "refatora"
- Ser descritivo e específico
- Mencionar o escopo quando relevante

## 🌿 Gerenciamento de Branches

### Nomenclatura de Branches

```
feature/<descrição>      → Novas funcionalidades
fix/<descrição>          → Correções de bugs
refactor/<descrição>     → Refatoração
docs/<descrição>         → Documentação
chore/<descrição>        → Tarefas de manutenção
hotfix/<descrição>       → Correções urgentes
```

### Exemplos

```bash
feature/tema-preferencias
feature/nfe-fiscal
fix/autenticacao-oauth
refactor/estrutura-modular
docs/guia-instalacao
chore/upgrade-dependencias
hotfix/crash-fatal
```

### Workflow Padrão

1. **Criar branch a partir de main**
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/minha-feature
   ```

2. **Desenvolver e commitar**
   ```bash
   git add .
   git commit -m "feat: adiciona nova funcionalidade"
   ```

3. **Manter branch atualizada com main**
   ```bash
   git fetch origin
   git rebase origin/main
   ```

4. **Push para remoto**
   ```bash
   git push origin feature/minha-feature
   ```

5. **Criar Pull Request no GitHub**
   - Descrever mudanças
   - Referenciar issues: `Closes #123`
   - Aguardar review

6. **Merge após aprovação**
   ```bash
   git checkout main
   git merge feature/minha-feature
   git push origin main
   ```

## 🔄 Fluxo de Desenvolvimento Multi-Submodule

### Desenvolvimento em Paralelo

Quando desenvolvendo em múltiplos submodules:

```bash
# 1. Criar branches em todos os submodules
cd backend && git checkout -b feature/nova-api && cd ..
cd frontend && git checkout -b feature/nova-ui && cd ..
cd mobile && git checkout -b feature/nova-tela && cd ..

# 2. Fazer commits em cada um
cd backend && git commit -m "feat: adiciona endpoint X" && cd ..
cd frontend && git commit -m "feat: adiciona componente Y" && cd ..

# 3. Atualizar referências no parent
git add backend frontend
git commit -m "chore: atualiza referências dos submodules com novas features"
```

### Sincronização de Branches

```bash
# Atualizar todos os submodules com branch específico
for dir in backend frontend mobile desktop; do
  (cd "$dir" && git checkout feature/sync && git pull origin feature/sync)
done
```

## 📊 Verificação de Status

### Verificar Status de Todos os Repositórios

```bash
# Status do repositório principal
git status

# Status de todos os submodules
git submodule foreach 'echo "=== $name ===" && git status'

# Verificar branches em todos os submodules
git submodule foreach 'echo "=== $name ===" && git branch -a'

# Verificar commits não enviados
git submodule foreach 'echo "=== $name ===" && git log origin/main..HEAD'
```

### Comando Prático Completo

```bash
# Criar alias para verificação rápida
git config --global alias.status-all '!git submodule foreach "echo === \$name === && git status"'

# Usar
git status-all
```

## 🔐 Segurança e Boas Práticas

### Nunca Commitar

❌ Secrets, senhas, tokens
❌ Arquivos `.env` não-example
❌ `node_modules`, `.venv`, `dist`
❌ Arquivos temporários ou de debug
❌ Dados sensíveis (CPF, email)

### Sempre Usar

✅ `.gitignore` apropriado
✅ `.env.example` sem valores reais
✅ SSH keys ou tokens gerenciados
✅ Verificar `git diff` antes de commit
✅ Mensagens de commit descritivas

### Exemplo de .gitignore Robusto

```gitignore
# Dependências
node_modules/
.venv/
__pycache__/

# Variáveis de ambiente
.env
.env.local
.env.*.local

# Arquivos de build
dist/
build/
.next/

# IDE
.vscode/
.idea/
*.swp

# OS
.DS_Store
Thumbs.db

# Logs
*.log
logs/

# Temporários
temp/
tmp/
*.tmp
```

## 🚀 Deploy & Produção

### Fluxo Recomendado

```
feature/nova-feature
    ↓ (Pull Request + Review)
main
    ↓ (Tag release)
v1.2.3
    ↓ (Build & Deploy)
production
```

### Criar Release Tag

```bash
# Criar tag anotada
git tag -a v1.2.3 -m "Release version 1.2.3"

# Enviar tags
git push origin --tags

# Ou fazer push de tag específica
git push origin v1.2.3
```

### Versionamento Semântico

**Formato:** `v{MAJOR}.{MINOR}.{PATCH}`

- **MAJOR:** Mudanças incompatíveis
- **MINOR:** Novas funcionalidades
- **PATCH:** Correções de bugs

**Exemplos:**
```
v1.0.0  → Primeira release
v1.1.0  → Nova feature
v1.1.1  → Bug fix
v2.0.0  → Breaking change
```

## 📚 Recursos Adicionais

### Comandos Úteis

```bash
# Ver histórico de commits
git log --oneline -10

# Ver diferenças
git diff main feature/nova-feature

# Desfazer último commit (local)
git reset --soft HEAD~1

# Ver quem modificou uma linha
git blame arquivo.ts

# Buscar commits por mensagem
git log --grep="autenticação"

# Rebase interativo
git rebase -i HEAD~5
```

### Links Úteis

- [GitHub Docs](https://docs.github.com)
- [Git Documentation](https://git-scm.com/doc)
- [Conventional Commits](https://www.conventionalcommits.org)
- [Semantic Versioning](https://semver.org)
- [GitHub CLI](https://cli.github.com)

## ✅ Checklist para Commits

Antes de fazer um commit:

- [ ] Código testado localmente
- [ ] Sem `console.log` ou `debugger`
- [ ] Sem secrets ou senhas
- [ ] Mensagem descritiva e em português
- [ ] Referencia issue/PR se aplicável
- [ ] Branch atualizada com main
- [ ] Sem conflitos de merge

## 🔗 Veja Também

- [ESPECIALISTA-FISCAL-TRIBUTARIO.md](./ESPECIALISTA-FISCAL-TRIBUTARIO.md) - Especialista em regras fiscais
- [../agents/](../agents/) - Agentes de automação disponíveis
- [CLAUDE.md](../../CLAUDE.md) - Instruções gerais para Claude no projeto

---

**Última atualização:** 09/12/2025
**Versão:** 1.0
**Responsável:** Sistema de Especialistas - GitHub & Submodules
