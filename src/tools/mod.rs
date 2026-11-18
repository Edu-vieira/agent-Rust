pub mod applications;
pub mod filesystem;
pub mod registry;

use crate::memory::SharedMemory;
use registry::{Tool, ToolRegistry};

pub fn build_registry(memory: SharedMemory) -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    // ==========================================
    // APPLICATIONS
    // ==========================================

    registry.register(Tool {
        name: "open_application".to_string(),
        description: "Abre um aplicativo permitido no Windows. Argumento: name.".to_string(),

        handler: Box::new(|args| {
            let name = args["name"]
                .as_str()
                .ok_or("Nome do aplicativo inválido".to_string())?;

            applications::open_application(name).map_err(|e| e.to_string())
        }),
    });

    // ==========================================
    // FILESYSTEM
    // ==========================================

    registry.register(Tool {
        name: "create_directory".to_string(),
        description:
            "Cria uma pasta dentro do espaço de trabalho do agente. Argumento: path relativo."
                .to_string(),

        handler: Box::new(|args| {
            let path = args["path"]
                .as_str()
                .ok_or("Caminho inválido".to_string())?;

            filesystem::create_directory(path).map_err(|e| e.to_string())
        }),
    });

    // Lista o conteúdo de um diretório.
    registry.register(Tool {
        name: "list_directory".to_string(),
        description: "Lista os arquivos e pastas existentes em um diretório do espaço de trabalho. Recebe: path.".to_string(),
        handler: Box::new(|args| {
            let path = args["path"]
                .as_str()
                .ok_or("O argumento 'path' é obrigatório.")?;

            filesystem::list_directory(path)
        }),
    });

    // Cria um arquivo e grava seu conteúdo.
    registry.register(Tool {
        name: "create_file".to_string(),
        description:
            "Cria um arquivo de texto dentro do espaço de trabalho. Recebe: path e content."
                .to_string(),
        handler: Box::new(|args| {
            let path = args["path"]
                .as_str()
                .ok_or("O argumento 'path' é obrigatório.")?;

            let content = args["content"]
                .as_str()
                .ok_or("O argumento 'content' é obrigatório.")?;

            filesystem::create_file(path, content)
        }),
    });

    // Lê o conteúdo de um arquivo.
    registry.register(Tool {
        name: "read_file".to_string(),
        description:
            "Lê o conteúdo de um arquivo de texto dentro do espaço de trabalho. Recebe: path."
                .to_string(),
        handler: Box::new(|args| {
            let path = args["path"]
                .as_str()
                .ok_or("O argumento 'path' é obrigatório.")?;

            filesystem::read_file(path)
        }),
    });
    // ==========================================
    // MEMORY SAVE
    // ==========================================

    let memory_save = memory.clone();

    registry.register(Tool {
        name: "memory_save".to_string(),
        description: "Armazena uma informação persistente. Argumentos: key e value.".to_string(),

        handler: Box::new(move |args| {
            let key = args["key"]
                .as_str()
                .ok_or("O argumento 'key' é obrigatório.")?;

            let value = args["value"]
                .as_str()
                .ok_or("O argumento 'value' é obrigatório.")?;

            if key.trim().is_empty() {
                return Err("A chave não pode estar vazia.".to_string());
            }

            let memory = memory_save
                .lock()
                .map_err(|_| "Falha ao acessar a memória.".to_string())?;

            memory.save(key, value).map_err(|e| e.to_string())?;
            Ok(format!("Informação '{}' salva com sucesso.", key))
        }),
    });

    // ==========================================
    // MEMORY GET
    // ==========================================

    let memory_get = memory.clone();

    registry.register(Tool {
        name: "memory_get".to_string(),
        description: "Recupera uma informação armazenada. Argumento: key.".to_string(),

        handler: Box::new(move |args| {
            let key = args["key"].as_str().ok_or("Chave inválida".to_string())?;

            let memory = memory_get
                .lock()
                .map_err(|_| "Falha ao acessar a memória.".to_string())?;

            match memory.get(key).map_err(|e| e.to_string())? {
                Some(value) => Ok(value),
                None => Ok(format!("Nenhuma informação encontrada para '{}'.", key)),
            }
        }),
    });

    // ==========================================
    // MEMORY SEARCH
    // ==========================================

    let memory_search = memory.clone();

    registry.register(Tool {
        name: "memory_search".to_string(),

        description: "Pesquisa informações armazenadas usando palavras ou expressões presentes nas chaves ou valores. Argumento: query.".to_string(),

        handler: Box::new(move |args| {

            let query = args["query"]
                .as_str()
                .ok_or("Consulta inválida".to_string())?;

            if query.trim().is_empty() {
                return Err("A consulta não pode ser vazia.".to_string());
            }

            let memory = memory_search
                .lock()
                .map_err(|_| "Falha ao acessar a memória.".to_string())?;

            let results = memory.search(query)
                .map_err(|e| e.to_string())?;

            if results.is_empty() {
                return Ok("Nenhuma informação encontrada.".to_string());
            }

            serde_json::to_string(&results)
                .map_err(|e| e.to_string())
        }),
    });

    // ==========================================
    // MEMORY ALL
    // ==========================================

    let memory_all = memory.clone();

    registry.register(Tool {
        name: "memory_all".to_string(),
        description: "Lista todas as informações armazenadas na memória. Não requer argumentos."
            .to_string(),

        handler: Box::new(move |_| {
            let memory = memory_all
                .lock()
                .map_err(|_| "Falha ao acessar a memória.".to_string())?;

            let memories = memory.all().map_err(|e| e.to_string())?;

            serde_json::to_string(&memories).map_err(|e| e.to_string())
        }),
    });

    // ==========================================
    // MEMORY DELETE
    // ==========================================

    registry.register(Tool {
        name: "memory_delete".to_string(),
        description: "Exclui uma informação da memória. Argumento: key.".to_string(),

        handler: Box::new(move |args| {
            let key = args["key"].as_str().ok_or("Chave inválida".to_string())?;

            let memory = memory
                .lock()
                .map_err(|_| "Falha ao acessar a memória.".to_string())?;

            match memory.delete(key).map_err(|e| e.to_string())? {
                true => Ok(format!("Informação '{}' excluída.", key)),
                false => Ok(format!("Informação '{}' não encontrada.", key)),
            }
        }),
    });

    registry
}
