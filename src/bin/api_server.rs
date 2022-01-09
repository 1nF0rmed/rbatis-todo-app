extern crate rbatis_todo_app;

use std::{env, sync::Arc};
use dotenv::dotenv;
use actix_web::{middleware, App, HttpServer};
use rbatis::rbatis::Rbatis;
use rbatis_todo_app::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let rb = Rbatis::new();
    rb.link(&database_url).await.expect("Should connect to database");
    let rb = Arc::new(rb);

    let serv = HttpServer::new(move || {
        App::new()
            .data(rb.to_owned())
            .wrap(middleware::Compress::default())
            .configure(routes)
    });
    serv.bind("0.0.0.0:8080")?.run().await
}
