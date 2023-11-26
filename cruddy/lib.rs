use actix_web::HttpResponse;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use serde::Serialize;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
