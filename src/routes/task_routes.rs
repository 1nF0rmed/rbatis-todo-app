use std::sync::Arc;

use actix_web::{web, HttpResponse};
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};

use crate::{
    models::Task,
    repository::task_repository::TaskRepository,
};

#[derive(Serialize, Deserialize)]
pub struct TaskRequest {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskCompleteRequest {
    completed: bool,
}

fn default_task_data() -> Vec<Task> {
    Vec::new()
}

#[derive(Serialize, Deserialize)]
pub struct StatusMessage {
    status: String,
    message: String,
    #[serde(default = "default_task_data")]
    data: Vec<Task>,
}

pub async fn create_task(task: web::Json<TaskRequest>, db_pool: web::Data<Arc<Rbatis>>) -> HttpResponse {
    let task_repo = TaskRepository {
        rb: db_pool.get_ref(),
    };

    let result = task_repo.create_task(task.title.clone(), task.description.clone()).await;

    match result {
        Ok(task) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "ok".to_string(),
                message: "".to_string(),
                data: vec![task],
            }),
        Err(_err) => {
            println!("Error: {}", _err);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "error".to_string(),
                message: "Unable to add task".to_string(),
                data: default_task_data(),
            })
        },
    }
}

pub async fn complete_task(
    task_id: web::Path<String>,
    task: web::Json<TaskCompleteRequest>,
    db_pool: web::Data<Arc<Rbatis>>
) -> HttpResponse {
    let task_repo = TaskRepository {
        rb: db_pool.get_ref(),
    };

    if !task.completed {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "error".to_string(),
                message: "Cannot update task completion to false".to_string(),
                data: default_task_data(),
            });
    }

    let result = task_repo.complete_task(task_id.to_string()).await;

    match result {
        Ok(task) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "ok".to_string(),
                message: "".to_string(),
                data: vec![task],
            }),
        Err(_err) => HttpResponse::Ok()
            .content_type("application/json")
            .json(StatusMessage {
                status: "error".to_string(),
                message: "Unable to complete task".to_string(),
                data: default_task_data(),
            }),
    }
}

#[cfg(test)]
mod tests {

    use std::{env, sync::Arc};

    use super::*;
    use actix_web::{test, web, App};
    use rbatis::crud::CRUD;

    #[actix_rt::test]
    async fn test_should_create_task() {
        let rb = Rbatis::new();
        rb.link(&env::var("DATABASE_URL").expect("Should get DATABASE_URL")).await.expect("Should connect to database");
        let rb = Arc::new(rb);

        let data = TaskRequest {
            title: "Adding a new PR".to_string(),
            description: "Update base repo to implement serde ro parse data".to_string(),
        };
        let mut app =
            test::init_service(App::new().data(rb.to_owned()).route("/tasks/create", web::post().to(create_task)))
                .await;
        let req = test::TestRequest::post()
            .uri("/tasks/create")
            .set_json(data)
            .to_request();
        let resp: StatusMessage = test::call_and_read_body_json(&mut app, req).await;

        // Assert
        assert_eq!(resp.status, "ok".to_string());

        // teardown
        rb.remove_by_column::<Task,_>("id", &resp.data[0].id).await.expect("Should have deleted the added record");
    }
}
