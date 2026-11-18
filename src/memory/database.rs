use rusqlite::{Connection, params};

pub struct Memory {
    connection: Connection,
}

impl Memory {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let database_path =
            std::env::var("AI_AGENT_MEMORY_DB").unwrap_or_else(|_| "agent_memory.db".to_string());
        let connection = Connection::open(database_path)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn save(&self, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "INSERT INTO memories (key, value)
             VALUES (?1, ?2)
             ON CONFLICT(key)
             DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, rusqlite::Error> {
        self.connection
            .query_row(
                "SELECT value FROM memories WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
    }

    pub fn all(&self) -> Result<Vec<(String, String)>, rusqlite::Error> {
        let mut statement = self
            .connection
            .prepare("SELECT key, value FROM memories ORDER BY id")?;

        let rows = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut memories = Vec::new();

        for row in rows {
            memories.push(row?);
        }

        Ok(memories)
    }

    pub fn delete(&self, key: &str) -> Result<bool, rusqlite::Error> {
        let affected = self
            .connection
            .execute("DELETE FROM memories WHERE key = ?1", params![key])?;

        Ok(affected > 0)
    }

    pub fn search(&self, query: &str) -> Result<Vec<(String, String)>, rusqlite::Error> {
        let stopwords = [
            "qual",
            "quais",
            "que",
            "como",
            "onde",
            "estamos",
            "utilizando",
            "utilizamos",
            "sobre",
            "para",
            "com",
            "uma",
            "uns",
            "das",
            "dos",
            "por",
            "meu",
            "minha",
            "nosso",
            "nossa",
            "de",
            "do",
            "da",
            "o",
            "a",
            "os",
            "as",
            "e",
            "em",
        ];

        let terms: Vec<String> = query
            .split_whitespace()
            .map(|word| {
                word.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .filter(|word| word.len() >= 3 && !stopwords.contains(&word.as_str()))
            .collect();

        if terms.is_empty() {
            return Ok(Vec::new());
        }

        let conditions = terms
            .iter()
            .map(|_| "(key LIKE ? OR value LIKE ?)")
            .collect::<Vec<_>>()
            .join(" OR ");

        let sql = format!(
            "SELECT key, value
         FROM memories
         WHERE {}
         ORDER BY id
         LIMIT 20",
            conditions
        );

        let mut statement = self.connection.prepare(&sql)?;

        let parameters: Vec<String> = terms
            .iter()
            .flat_map(|term| {
                let pattern = format!("%{}%", term);
                [pattern.clone(), pattern]
            })
            .collect();

        let rows = statement.query_map(rusqlite::params_from_iter(parameters.iter()), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut results = Vec::new();

        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }
}

use rusqlite::OptionalExtension;
