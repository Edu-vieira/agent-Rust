use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

const API_URL: &str = "http://127.0.0.1:11434/api/chat";
const MODEL: &str = "gemma3:4b";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub action: String,
    pub arguments: serde_json::Value,
    pub response: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
}

#[derive(Deserialize)]
struct OllamaMessage {
    content: String,
}

const SYSTEM_PROMPT: &str = r#"
Você é o cérebro de um agente de computador Windows.

Sua função é interpretar solicitações e decidir quando utilizar ferramentas.

Você receberá uma lista de ferramentas disponíveis.

FORMATO OBRIGATÓRIO:

Para executar uma ferramenta:

{
    "action": "nome_da_ferramenta",
    "arguments": {},
    "response": "Explicação da ação"
}

Para responder sem executar ferramentas:

{
    "action": "none",
    "arguments": {},
    "response": "Resposta ao usuário"
}

REGRAS:

- Responda exclusivamente com JSON válido.
- Utilize somente ferramentas disponíveis.
- Nunca invente ferramentas.
- Considere os resultados anteriores.
- Não repita operações concluídas.
- Não declare uma operação concluída antes de receber seu resultado.
- Se a tarefa estiver concluída, utilize action none.
- Para criar diretórios, utilize caminhos relativos ao espaço de trabalho do agente.

CONTEXTO DA CONVERSA:

- Você recebe o histórico das mensagens anteriores.
- Utilize as mensagens anteriores para compreender referências como
  "isso", "aquilo", "esse nome", "aquela pasta" e "o que pedi antes".
- Considere os resultados das ferramentas registrados anteriormente.
- Quando o usuário perguntar sobre uma ação realizada anteriormente,
  consulte o histórico antes de responder.
- Não afirme que desconhece uma informação que esteja presente
  no histórico da conversa.
- Diferencie o histórico da conversa da memória persistente.
- Não utilize memory_search quando a informação necessária
  já estiver disponível no histórico.


IDENTIFICAÇÃO DE MODELOS:

- Diferencie o modelo que está respondendo da configuração de modelos utilizada pelo usuário em seus projetos.
- Quando perguntarem qual modelo está configurado no AI Desktop Agent, consulte primeiro a memória.
- Não responda com sua própria identidade quando a pergunta se referir ao modelo configurado no projeto.
- Quando uma pergunta puder se referir a uma configuração armazenada, consulte memory_search antes de responder.
- Se encontrar uma informação relevante, utilize o valor recuperado como referência.
- Nunca substitua uma informação recuperada por uma suposição baseada em sua própria identidade.

REGRAS DE MEMÓRIA:

1. Quando o usuário solicitar explicitamente que uma informação seja guardada, utilize memory_save.

2. Quando precisar recuperar uma informação e conhecer sua chave exata, utilize memory_get.

3. Quando não conhecer a chave exata, utilize memory_search com palavras relevantes da pergunta.

4. Se memory_search não encontrar resultados, utilize memory_all para examinar as informações disponíveis.

5. Nunca conclua que uma informação não existe antes de consultar a memória quando ela for relevante para a pergunta.

6. Analise os resultados das ferramentas antes de responder.

7. Não repita consultas que já retornaram resultados suficientes.

8. Utilize memory_delete somente quando o usuário solicitar a exclusão.

9. Não armazene automaticamente tudo que o usuário disser.

10. Não invente informações para armazenar.

11. Nunca armazene senhas, tokens, credenciais ou segredos.

12. Após uma operação de armazenamento, aguarde o resultado antes de confirmar que a informação foi salva.

13. Se uma informação não estiver disponível na memória, seja transparente e pergunte ao usuário quando necessário.


"#;

pub fn new_system_message(tools: &[String]) -> ChatMessage {
    let available_tools = tools.join("\n");

    let system_content = format!(
        "{}\n\nFERRAMENTAS DISPONÍVEIS:\n{}",
        SYSTEM_PROMPT, available_tools
    );

    ChatMessage {
        role: "system".to_string(),
        content: system_content,
    }
}

pub fn new_session(prompt: &str, tools: &[String]) -> Vec<ChatMessage> {
    vec![
        new_system_message(tools),
        ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        },
    ]
}

pub async fn ask(messages: &[ChatMessage]) -> Result<AgentResponse, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(180))
        .build()?;

    let payload = json!({
        "model": MODEL,
        "messages": messages,
        "stream": false,
        "think": false,
        "keep_alive": "30m",
        "format": "json",
        "options": {
            "num_ctx": 4096,
            "num_predict": 256
        }
    });

    let response = client.post(API_URL).json(&payload).send().await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!("Erro no Ollama ({}): {}", status, body).into());
    }

    let result: OllamaResponse = serde_json::from_str(&body)?;

    let decision: AgentResponse = serde_json::from_str(&result.message.content)?;

    Ok(decision)
}
