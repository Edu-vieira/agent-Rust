use crate::conversation::Conversation;
use crate::llm;
use crate::llm::ChatMessage;
use crate::memory::SharedMemory;

const MAX_STEPS: usize = 5;

pub async fn handle_message(
    message: &str,
    memory: SharedMemory,
    conversation: &mut Conversation,
) -> Result<String, Box<dyn std::error::Error>> {
    // Cria o registro com acesso à memória compartilhada.
    let registry = crate::tools::build_registry(memory);

    // Obtém as ferramentas disponíveis.
    let descriptions = registry.descriptions();

    // Cria a mensagem de sistema.
    let system_message = llm::new_system_message(&descriptions);

    // Adiciona a nova mensagem ao histórico existente.
    conversation.push(ChatMessage {
        role: "user".to_string(),
        content: message.to_string(),
    });

    // Ciclo de raciocínio.
    for step in 0..=MAX_STEPS {
        // Recupera todo o histórico.
        let history = conversation.messages_with_system(system_message.clone());

        // Envia o contexto completo ao modelo.
        let decision = llm::ask(&history).await?;

        // Verifica se o agente terminou.
        if decision.action == "none" {
            // Registra a resposta final em linguagem natural.
            conversation.push(ChatMessage {
                role: "assistant".to_string(),
                content: decision.response.clone(),
            });

            return Ok(decision.response);
        }

        // Registra a decisão de executar uma ferramenta.
        conversation.push(ChatMessage {
            role: "assistant".to_string(),
            content: serde_json::to_string(&decision)?,
        });

        // Impede operações além do limite.
        if step == MAX_STEPS {
            return Ok(format!("Limite de {} operações atingido.", MAX_STEPS));
        }

        // Executa a ferramenta solicitada.
        let result = registry.execute(&decision.action, &decision.arguments);

        // Registra o resultado no histórico.
        match result {
            Ok(output) => {
                conversation.push(ChatMessage {
                    role: "user".to_string(),
                    content: format!("Resultado da ferramenta {}: {}", decision.action, output),
                });
            }

            Err(error) => {
                conversation.push(ChatMessage {
                    role: "user".to_string(),
                    content: format!(
                        "A ferramenta {} falhou: {}. \
                         Considere esse resultado antes de continuar.",
                        decision.action, error
                    ),
                });
            }
        }
    }

    Ok(format!("Limite de {} operações atingido.", MAX_STEPS))
}
