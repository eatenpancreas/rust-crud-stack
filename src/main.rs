mod models;
mod routes;
mod schema;

use actix_web::*;
use actix_web::web::{Data};
use serde::Serialize;
use cruddy::{DBList, DBPool};
use crate::models::todo::{Todo};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_list: Data<DBList<Todo>> = Data::new(DBList::new(DBPool::new()));
    
    HttpServer::new(move ||
        App::new()
            .app_data(db_list.clone())
            .configure(routes::api)
            .service(routes::healthcheck)
            .default_service(web::route().to(routes::not_found))
            .wrap(middleware::Logger::default())
    ).bind((std::env::var("ADDRESS").unwrap(), 8080))?
     .run()
     .await
}