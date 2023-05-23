use crate::Conf;
use sqlx::postgres::PgPoolOptions;
pub use sqlx::{Pool, Postgres};

pub async fn get_pool(conf: &Conf) -> Pool<Postgres> {
    let dns = format!(
        "postgres://{}:{}@{}/{}",
        conf.db.user, conf.db.password, conf.db.host, conf.db.db_name
    );
    let pool = PgPoolOptions::new()
        .max_connections(conf.db.max_open_conns)
        // use your own credentials
        .connect(&dns)
        .await
        .expect("couldn't connect to the database");
    pool
}
