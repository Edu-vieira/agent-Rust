# AI Desktop Agent

Agente de terminal para Windows, escrito em Rust. Ele conversa com um servidor Ollama local, pode abrir uma lista limitada de aplicativos, manipular arquivos em um workspace próprio e guardar informações em uma base SQLite.

## Requisitos

- Windows 10 ou 11.
- Rust estável com suporte à edição 2024, instalado pelo [rustup](https://rustup.rs/).
- [Ollama para Windows](https://ollama.com/download).
- Um modelo compatível instalado no Ollama. O padrão é `gemma3:4b`.

## Preparar e executar

No PowerShell, na pasta do projeto:

```powershell
ollama pull gemma3:4b
cargo run
```

O Ollama precisa estar disponível enquanto o agente estiver em execução. Para gerar um executável otimizado, use `cargo build --release`; o binário ficará em `target\release\ai-desktop-agent.exe`.

## Configuração

As configurações são opcionais e usam estes padrões:

| Variável | Padrão | Uso |
| --- | --- | --- |
| `AI_AGENT_OLLAMA_URL` | `http://127.0.0.1:11434/api/chat` | URL completa do endpoint `/api/chat` do Ollama |
| `AI_AGENT_MODEL` | `gemma3:4b` | Nome do modelo instalado no Ollama |
| `AI_AGENT_MEMORY_DB` | `agent_memory.db` | Caminho do arquivo SQLite de memória |

Exemplo para a sessão atual do PowerShell:

```powershell
$env:AI_AGENT_MODEL = "llama3.2:3b"
$env:AI_AGENT_OLLAMA_URL = "http://127.0.0.1:11434/api/chat"
cargo run
```

## Uso

- Digite uma solicitação em português no terminal.
- `limpar conversa` apaga o histórico temporário da sessão.
- `sair` encerra o programa. Ctrl+C também pode ser usado.
- Ao encerrar a entrada padrão, o programa finaliza normalmente.

As ferramentas de arquivos usam `%USERPROFILE%\AI-Agent-Workspace`. Elas aceitam caminhos relativos a essa pasta, não sobrescrevem arquivos existentes e restringem a leitura e a escrita ao workspace. Os aplicativos que podem ser abertos estão limitados a Notepad, Calculadora, Paint e Explorador de Arquivos.

A memória é persistente entre execuções. `memory_save` cria ou atualiza uma informação identificada por chave; as ferramentas disponíveis também permitem consultar, pesquisar, listar e apagar informações.

## Limites conhecidos

- O agente depende de um servidor Ollama local e de um modelo baixado previamente.
- O agente faz no máximo cinco chamadas de ferramenta por solicitação.
- O histórico da conversa é mantido apenas durante a execução atual e limitado às 20 mensagens mais recentes.
- `models.json` é um catálogo de modelos Gemini e não configura o provedor atual, que é Ollama.
