use chrono::NaiveDateTime;
use chrono::prelude::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use cruddy::DBPool;
use crate::schema::todos::todos::table as todos;
use crate::schema::todos::todos as todos_table;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = todos_table)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<Todo> for NewTodo {
    fn into(self) -> Todo {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            title: self.title,
            description: self.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        todo
    }
}

impl Todo {
    pub fn insert(self, pool: &DBPool) -> std::io::Result<Todo> {
        diesel::insert_into(todos)
            .values(self.clone())
            .execute(&mut pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(self)
    }
    
    pub fn get(pool: &DBPool) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn get_by_id(id: &str, pool: &DBPool) -> Option<Todo> {
        Some(todos
            .find(id)
            .get_result::<Todo>(&mut pool.get().unwrap())
            .expect("Error loading todo by id")
        )
    }

    pub fn update_by_id(id: &str, pool: &DBPool, update_todo: UpdateTodo) -> Option<Todo> {
        let mut todo = Self::get_by_id(id, pool).expect("Could not find todo");
        let mut updated = false;
        
        if let Some(t) = update_todo.title { todo.title = t; updated = true; }
        if let Some(d) = update_todo.description { todo.description = Some(d); updated = true; }
        
        if updated { todo.updated_at = Utc::now().naive_utc(); }
        
        let todo = diesel::update(todos.find(id))
            .set(&todo)
            .get_result::<Todo>(&mut pool.get().unwrap())
            .expect("Error updating todo by id");
        
        Some(todo)
    }
    
    pub fn delete_by_id(id: &str, pool: &DBPool) -> usize {
        diesel::delete(todos.find(id))
            .execute(&mut pool.get().unwrap())
            .unwrap_or(0)
    }
}