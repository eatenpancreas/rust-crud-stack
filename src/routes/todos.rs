use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web::web::{Data, Json};
use chrono::Utc;
use cruddy::{DBPool};
use crate::models::todo::{NewTodo, Todo, UpdateTodo};

#[post("/todos")]
pub async fn create_todo(pool: Data<DBPool>, new_todo: Json<NewTodo>) -> HttpResponse {
    let into: Todo = new_todo.0.into();
    match into.insert(pool.get_ref()) {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(pool: Data<DBPool>) -> HttpResponse {
    HttpResponse::Ok().json(Todo::get(pool.get_ref()))
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(pool: Data<DBPool>, id: web::Path<String>) -> HttpResponse {
    match Todo::get_by_id(&*id.into_inner(), &pool) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(pool: Data<DBPool>, id: web::Path<String>, updated_todo: Json<UpdateTodo>) -> HttpResponse {
    match Todo::update_by_id(&*id.into_inner(), &pool, updated_todo.0) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(pool: Data<DBPool>, id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(Todo::delete_by_id(&*id.into_inner(), &pool))
}