use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web::web::{Data, Json};
use chrono::Utc;
use cruddy::DBList;
use crate::models::todo::{NewTodo, Todo, UpdateTodo};

#[post("/todos")]
pub async fn create_todo(db: Data<DBList<Todo>>, new_todo: Json<NewTodo>) -> HttpResponse {
    match db.add_from_json(new_todo) {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: Data<DBList<Todo>>) -> HttpResponse {
    HttpResponse::Ok().json(db.clone_out())
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: Data<DBList<Todo>>, id: web::Path<String>) -> HttpResponse {
    let id_str = id.into_inner();
    
    match db.get_by(|todo| todo.id == id_str) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(db: Data<DBList<Todo>>, id: web::Path<String>, updated_todo: Json<UpdateTodo>) -> HttpResponse {
    let id_str = id.into_inner();
    
    match db.update_by(|todo| todo.id == id_str, move |mut todo| {
        let mut updated = false;
        if let Some(desc) = &updated_todo.description { 
            todo.description = Some(desc.clone());
            updated = true;
        }
        if let Some(title) = &updated_todo.title { 
            todo.title = title.clone();
            updated = true;
        }
        if updated { todo.updated_at = Utc::now(); }
        todo
    }) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<DBList<Todo>>, id: web::Path<String>) -> HttpResponse {
    let id_str = id.into_inner();
    
    match db.delete_by(|todo| todo.id == id_str) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}