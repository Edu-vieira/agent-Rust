mod agent;
mod conversation;
mod llm;
mod memory;
mod tools;

use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("================================");
    println!("       AI DESKTOP AGENT");
    println!("================================");

    // Inicializa a memória persistente.
    let memory = memory::create_shared_memory()?;

    // Inicializa o histórico temporário.
    let mut conversation = conversation::Conversation::new();

    loop {
        print!("\nVocê: ");
        io::stdout().flush()?;

        let mut message = String::new();

        if io::stdin().read_line(&mut message)? == 0 {
            println!("\nEntrada encerrada. Finalizando o agente...");
            break;
        }

        let message = message.trim();

        if message.is_empty() {
            println!("Nenhuma mensagem informada.");
            continue;
        }

        if message.eq_ignore_ascii_case("sair") {
            println!("\nEncerrando o agente...");
            break;
        }

        if message.eq_ignore_ascii_case("limpar conversa") {
            conversation.clear();

            println!("\nHistórico da conversa limpo.");
            continue;
        }

        println!("\nAgente: processando...");

        match agent::handle_message(message, memory.clone(), &mut conversation).await {
            Ok(response) => {
                println!("\nAgente: {}\n", response);
            }

            Err(error) => {
                eprintln!("\nErro: {}\n", error);
            }
        }
    }

    Ok(())
}
