mod models;
mod routes;
mod schema;

use actix_web::*;
use actix_web::web::{Data};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Serialize;
use cruddy::{DBPool};
use crate::models::todo::{Todo};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    diesel_migrations::embed_migrations!();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_list: Data<DBPool> = Data::new(Pool::new(manager).unwrap());
    
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