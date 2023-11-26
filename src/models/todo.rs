
use chrono::prelude::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

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

//, Insertable , AsChangeset
#[derive(Serialize, Debug, Clone, Queryable)]
#[diesel(table_name = crate::schema::todos)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<Todo> for NewTodo {
    fn into(self) -> Todo {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            title: self.title,
            description: self.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        todo
    }
}