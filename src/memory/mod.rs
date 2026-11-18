pub mod database;

use std::sync::{Arc, Mutex};

pub use database::Memory;

pub type SharedMemory = Arc<Mutex<Memory>>;

pub fn create_shared_memory() -> Result<SharedMemory, rusqlite::Error> {
    let memory = Memory::new()?;

    Ok(Arc::new(Mutex::new(memory)))
}
