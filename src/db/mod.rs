use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::{env, time::Duration};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    r2d2::Pool::builder()
        .max_size(16)
        .min_idle(Some(0))
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("failed to create DB pool")
}
