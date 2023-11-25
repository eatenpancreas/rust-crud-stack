use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
const DB_URL: &str = env!("DATABASE_URL");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .route("/", web::get().to(HttpResponse::Ok))
    ).bind(("127.0.0.1", 8080))?.run().await
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    DB_URL
}
// 
// fn set_database() -> Result<(), PostgresError> {
//     let client = Client::connect()
// }
