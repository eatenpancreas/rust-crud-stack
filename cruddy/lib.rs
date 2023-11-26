use std::env::VarError;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use actix_web::web::Json;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;

#[derive(Clone)]
pub struct DBPool(r2d2::Pool<ConnectionManager<PgConnection>>);

impl DBPool {
    pub fn new() -> DBPool {
        let database_url = std::env::var("DATABASE_URL").ok().unwrap();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        DBPool(r2d2::Pool::builder()
            .build(manager).unwrap()
        )
    }
}

#[derive(Clone)]
pub struct DBList<T>(Arc<Mutex<Vec<T>>>) where T: Clone;

impl <'i, T> DBList<T> where T: Clone {
    pub fn new() -> DBList<T> {
        DBList(Arc::new(Mutex::new(vec![])))
    }

    pub fn add(&self, new: T) -> std::io::Result<T> {
        self.0.lock().unwrap().push(new.clone());
        Ok(new)
    }

    pub fn clone_out(&self) -> Vec<T> {
        self.0.lock().unwrap().clone()
    }

    pub fn update_by(&self, predicate: impl FnMut(&T) -> bool, updater: impl Fn(T) -> T) -> Option<T> {
        let mut arr = self.0.lock().unwrap();
        let index = arr.iter().position(predicate)?;
        arr[index] = updater(arr[index].clone());
        Some(arr[index].clone())
    }

    pub fn delete_by(&self, predicate: impl FnMut(&T) -> bool) -> Option<T> {
        let mut arr = self.0.lock().unwrap();
        let index = arr.iter().position(predicate)?;
        Some(arr.remove(index))
    }

    pub fn get_by(&self, predicate: impl FnMut(&&T) -> bool) -> Option<T> {
        self.0.lock().unwrap().iter().find(predicate).cloned()
    }

    pub fn add_from_json<J>(&self, new: Json<J>) -> std::io::Result<T> where J: Into<T> {
        self.add(new.into_inner().into())
    }
}