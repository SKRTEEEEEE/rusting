//En el módulo tasks se representan las tareas y cómo se van a guardar y se va a acceder a ellas.
//3. Task, utilizando serde
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}


//1. estructura de Task básica
// use chrono::{DateTime, Utc};

// #[derive(Debug)]
// pub struct Task {
//     pub text: String,
//     pub created_at: DateTime<Utc>,
// }

//2. Implementar un método para crear instancias de nuevas tareas. Las tareas siempre van a llevar una marca de tiempo con la fecha y la hora actuales.

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
    //4. Añadir funciones
    use std::io::Result;
    use std::path::PathBuf;

    pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> { ... }

    pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> { ... }

    pub fn list_tasks(journal_path: PathBuf) -> Result<()> { ... }
}