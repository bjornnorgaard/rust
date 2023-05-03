use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub complete: bool,
    pub created: DateTime<Utc>,
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Todo {{ id: {}, title: '{}', complete: {}, created: {} }}",
            self.id, self.title, self.complete, self.created
        )
    }
}

pub struct Service {
    db: PgPool,
}

impl Service {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL")
            .expect("env var DATABASE_URL not set");

        let conn = PgPool::connect(&url).await
            .expect("new db connection");

        sqlx::migrate!("./migrations").run(&conn).await
            .expect("migrations applied");

        Self {
            db: conn,
        }
    }

    pub async fn create(&self, title: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query!("insert into todos (id, title) values ($1, $2)", Uuid::new_v4(), title)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let res: Vec<Todo> = sqlx::query_as!(Todo, "select id, title, complete, created from todos")
            .fetch_all(&self.db)
            .await?;

        return Ok(res);
    }

    pub async fn get_one(&self, id: Uuid) -> Result<Todo, Box<dyn Error>> {
        let res: Todo = sqlx::query_as!(
                Todo,
                "select id, title, complete, created from todos where id = $1 limit 1",
                id,
            ).fetch_one(&self.db).await?;

        return Ok(res);
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        sqlx::query!("delete from todos where id = $1", id).execute(&self.db).await?;

        return Ok(());
    }
}
