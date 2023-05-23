use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres};

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub name: String,
    pub locale: String,
    pub display_name: String,
}

pub async fn get_applications(
    pool: &sqlx::Pool<Postgres>,
) -> Result<Vec<Application>, sqlx::Error> {
    let apps =
        sqlx::query_as::<_, Application>("select id, name, locale, display_name from application")
            .fetch_all(pool)
            .await
            .unwrap();
    Ok(apps)
}

// pub async fn add_application(
//     pool: &sqlx::Pool<Postgres>,
//     name: &str,
//     locale: &str,
//     display_name: &str,
// ) -> Result<i64, sqlx::Error> {
//     let row: (i64,) = sqlx::query_as(
//         "insert into application (name, locale, display_name) values ($1, $2, $3) returning id",
//     )
//     .bind(&name)
//     .bind(&locale)
//     .bind(&display_name)
//     .fetch_one(pool)
//     .await?;
//     Ok(row.0)
// }
