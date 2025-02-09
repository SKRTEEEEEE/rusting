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
    //La función add_task tiene que anexar un nuevo valor Task a una colección de tareas posiblemente existente que está codificada en un archivo JSON. Por lo tanto, antes de insertar una tarea en esa colección, primero debemos leer ese archivo y ensamblar un vector de tareas a partir de su contenido.
    pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> { 
         // Open the file.
        let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

        // Consume the file's contents as a vector of tasks.
        let mut tasks: Vec<Task> = collect_tasks(&file)?;

        // Write the modified task list back into the file.
        tasks.push(task);
        serde_json::to_writer(file, &tasks)?;

        Ok(())

     }

     use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};  // Include the `Error` type.

     fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
        file.seek(SeekFrom::Start(0))?; // Rewind the file before.
        let tasks = match serde_json::from_reader(file) {
            Ok(tasks) => tasks,
            Err(e) if e.is_eof() => Vec::new(),
            Err(e) => Err(e)?,
        };
        file.seek(SeekFrom::Start(0))?; // Rewind the file after.
        Ok(tasks)
    }

    pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
        // Open the file.
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(journal_path)?;
    
        // Consume file's contents as a vector of tasks.
        let mut tasks = collect_tasks(&file)?;
    
        // Try to remove the task.
        if task_position == 0 || task_position > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
        tasks.remove(task_position - 1);
    
        // Write the modified task list back into the file.
        file.set_len(0)?;
        serde_json::to_writer(file, &tasks)?;
        Ok(())
    }

    pub fn list_tasks(journal_path: PathBuf) -> Result<()> { ... }
}