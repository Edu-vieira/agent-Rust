use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const WORKSPACE_NAME: &str = "AI-Agent-Workspace";

/// Retorna o diretório de trabalho do agente.
fn workspace_root() -> Result<PathBuf, String> {
    let user_profile = std::env::var("USERPROFILE")
        .map_err(|_| "Não foi possível localizar o perfil do usuário.".to_string())?;

    let root = PathBuf::from(user_profile).join(WORKSPACE_NAME);

    fs::create_dir_all(&root)
        .map_err(|e| format!("Erro ao preparar o diretório de trabalho: {}", e))?;

    fs::canonicalize(&root).map_err(|e| format!("Erro ao acessar o diretório de trabalho: {}", e))
}

/// Valida um caminho relativo e impede acesso fora do workspace.
fn resolve_workspace_path(path: &str) -> Result<PathBuf, String> {
    let relative_path = Path::new(path);

    if path.trim().is_empty() {
        return Err("O caminho não pode estar vazio.".to_string());
    }

    if relative_path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("Informe um caminho relativo válido, sem sair do workspace.".to_string());
    }

    let root = workspace_root()?;
    Ok(root.join(relative_path))
}

/// Confere se um caminho existente está dentro do workspace.
fn validate_existing_path(path: &Path) -> Result<PathBuf, String> {
    let root = workspace_root()?;

    let canonical_path =
        fs::canonicalize(path).map_err(|e| format!("Erro ao acessar o caminho: {}", e))?;

    if !canonical_path.starts_with(&root) {
        return Err("Acesso fora do diretório de trabalho não permitido.".to_string());
    }

    Ok(canonical_path)
}

/// Cria um diretório dentro do workspace.
pub fn create_directory(path: &str) -> Result<String, String> {
    let directory = resolve_workspace_path(path)?;

    fs::create_dir_all(&directory).map_err(|e| format!("Erro ao criar diretório: {}", e))?;

    let directory = validate_existing_path(&directory)?;

    Ok(format!(
        "Diretório criado com sucesso: {}",
        directory.display()
    ))
}

/// Lista os arquivos e diretórios de uma pasta.
pub fn list_directory(path: &str) -> Result<String, String> {
    let directory = resolve_workspace_path(path)?;
    let directory = validate_existing_path(&directory)?;

    if !directory.is_dir() {
        return Err("O caminho informado não é um diretório.".to_string());
    }

    let entries =
        fs::read_dir(&directory).map_err(|e| format!("Erro ao listar diretório: {}", e))?;

    let mut items = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Erro ao acessar item do diretório: {}", e))?;

        let name = entry.file_name().to_string_lossy().into_owned();
        let file_type = entry
            .file_type()
            .map_err(|e| format!("Erro ao identificar tipo do item: {}", e))?;

        if file_type.is_dir() {
            items.push(format!("[PASTA] {}", name));
        } else if file_type.is_file() {
            items.push(format!("[ARQUIVO] {}", name));
        } else {
            items.push(format!("[OUTRO] {}", name));
        }
    }

    items.sort();

    if items.is_empty() {
        return Ok("O diretório está vazio.".to_string());
    }

    Ok(format!(
        "Conteúdo de {}:\n{}",
        directory.display(),
        items.join("\n")
    ))
}

/// Cria um arquivo de texto e grava seu conteúdo.
pub fn create_file(path: &str, content: &str) -> Result<String, String> {
    let file_path = resolve_workspace_path(path)?;

    let parent = file_path
        .parent()
        .ok_or_else(|| "Não foi possível identificar a pasta do arquivo.".to_string())?;

    fs::create_dir_all(parent)
        .map_err(|e| format!("Erro ao preparar diretório do arquivo: {}", e))?;

    let parent = validate_existing_path(parent)?;
    let file_name = file_path
        .file_name()
        .ok_or_else(|| "Nome de arquivo inválido.".to_string())?;

    let safe_file_path = parent.join(file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&safe_file_path)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                "Esse arquivo já existe. Não foi sobrescrito.".to_string()
            } else {
                format!("Erro ao criar arquivo: {}", e)
            }
        })?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Erro ao gravar conteúdo: {}", e))?;

    Ok(format!(
        "Arquivo criado com sucesso: {}",
        safe_file_path.display()
    ))
}

/// Lê o conteúdo de um arquivo de texto.
pub fn read_file(path: &str) -> Result<String, String> {
    let file_path = resolve_workspace_path(path)?;
    let file_path = validate_existing_path(&file_path)?;

    if !file_path.is_file() {
        return Err("O caminho informado não é um arquivo.".to_string());
    }

    fs::read_to_string(&file_path).map_err(|e| format!("Erro ao ler arquivo: {}", e))
}
