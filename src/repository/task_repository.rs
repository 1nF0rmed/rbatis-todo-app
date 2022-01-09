use crate::error::{Result, TodoAppError};
use crate::models::Task;

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use uuid::Uuid;

pub struct TaskRepository<'a> {
    pub rb: &'a Rbatis,
}

impl TaskRepository<'_> {
    pub async fn create_task(&self, title: String, description: String) -> Result<Task> {
        let task_id = Uuid::new_v4().to_string();

        let new_task = Task {
            id: Some(task_id.clone()),
            title: Some(title),
            descp: Some(description),
            completed: Some(false)
        };
        self.rb.save(&new_task, &[]).await?;

        // Not sure if I'm doing it right, makes no sense to Result->Option here?
        let result:Option<Task> = self.rb.fetch_by_column("id", &task_id).await?;

        match result {
            Some(task) => Ok(task),
            None => Err(TodoAppError::RepositoryError)
        }
    }

    pub async fn complete_task(&self, id: String) -> Result<Task> {
        let task_update = Task {
            id: None,
            title: None,
            descp: None,
            completed: Some(true)
        };
        self.rb.update_by_column("id", &task_update).await?;

        // Not sure if I'm doing it right, makes no sense to Result->Option here?
        let result:Option<Task> = self.rb.fetch_by_column("id", &id).await?;

        match result {
            Some(task) => Ok(task),
            None => Err(TodoAppError::RepositoryError)
        }
    }
}
