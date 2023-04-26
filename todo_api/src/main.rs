use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use std::env;
use std::error::Error;
use tokio::main;

#[allow(dead_code)]
#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("failed to load dotenv");
    let url = env::var("DATABASE_URL").expect("env var DATABASE_URL not set");
    let conn = PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&conn).await?;

    create(&conn, "walk the dog").await?;

    // let todos = get_all(&conn).await?;

    // done
    // delete

    Ok(())
}

#[derive(Debug)]
struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub complete: bool,
    pub created: DateTime<Utc>,
}

/*async fn get_all(conn: &PgPool) -> Result<Vec<Todo>, Box<dyn Error>> {
    let query = sqlx::query_as!(Todo, r"select id, title, complete, created from todos");
    let result = query.fetch_all(conn).await?;
    return Ok(result);
}*/

async fn create(conn: &PgPool, title: &str) -> Result<(), Box<dyn Error>> {
    sqlx::query("insert into todos (id, title) values ($1, $2)")
        .bind(uuid::Uuid::new_v4())
        .bind(title)
        .execute(conn)
        .await?;

    Ok(())
}
