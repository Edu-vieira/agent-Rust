use std::process::Command;

pub fn open_application(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let executable = match name.to_lowercase().as_str() {
        "notepad" | "bloco de notas" => "notepad.exe",
        "calculator" | "calculadora" => "calc.exe",
        "paint" | "mspaint" => "mspaint.exe",
        "explorer" | "explorador de arquivos" => "explorer.exe",

        _ => {
            return Err(format!("Aplicativo não permitido: {}", name).into());
        }
    };

    Command::new(executable).spawn()?;

    Ok(format!("Aplicativo '{}' iniciado.", name))
}
