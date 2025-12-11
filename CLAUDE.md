# CLAUDE.md

## ⛔ Política de Commits

**IMPORTANTE**: Nunca fazer commit ou push sem autorização explícita do usuário (Lee Chardes).

Antes de qualquer operação git:
1. ✋ **SEMPRE pergunte** se pode commitar
2. ✋ **SEMPRE pergunte** se pode fazer push
3. ✋ **SEMPRE pergunte** se pode fazer merge
4. ❌ **NUNCA** execute git operations sem permissão

Exceção: Operações de leitura (git status, git log, git diff) são permitidas.

---

## ⚠️ Importante: Comandos de Substituição

### Evitar Comandos que Pedem Autorização
- **NÃO USAR**: `$(date)`, `$(comando)` ou outras substituições
- **NÃO USAR**: redirecionamento (>, >>)
- **NÃO USAR**: pipe (|)
- **NÃO USAR**: variáveis com $VARIAVEL
- Esses comandos ativam pedido de permissão e interrompem o fluxo

### Como Obter Data/Hora Corretamente
```bash
# ❌ ERRADO (pede autorização):
echo "[$(date '+%H:%M:%S')] Log message"

# ✅ CORRETO (sem autorização):
date '+%H:%M:%S'
# Resultado: 19:01:12
echo "[19:01:12] Log message"  # Usar o valor manualmente
```

## 🛠️ Ferramentas com Permissão Total
Todas as ferramentas abaixo estão pré-autorizadas:
- Read(**), Write(**), Edit(**), MultiEdit(**)
- Glob(**), Grep(**), LS(**)
- NotebookEdit(**), TodoWrite(**), Task(**)
- WebFetch(**), WebSearch(**)
- BashOutput(**), KillBash(**), ExitPlanMode(**)
- Bash(echo*), Bash(*)
