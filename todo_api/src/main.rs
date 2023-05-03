use std::error::Error;

use dotenv::dotenv;
use tokio::main;

mod todos;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("failed to load dotenv");

    let service = todos::Service::new().await;
    service.create("Walk the dog").await?;
    let todos = service.get_all().await?;
    for todo in todos {
        let t = service.get_one(todo.id).await?;
        service.delete(t.id).await?;
    }

    Ok(())
}
