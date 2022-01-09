use actix_web::{web, Responder};

use self::task_routes::{create_task, complete_task};

pub mod task_routes;

async fn health_check() -> impl Responder {
    "Welcome!"
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
    cfg.route("/tasks/create", web::post().to(create_task));
    cfg.route("/tasks/{task_id}/complete", web::patch().to(complete_task));
}