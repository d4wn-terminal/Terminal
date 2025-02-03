use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = "postgres://user:password@localhost/d4wn";
    let pool = PgPoolOptions::new().connect(database_url).await?;
    Ok(pool)
}
