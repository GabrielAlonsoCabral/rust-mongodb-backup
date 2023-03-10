use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let mut client_options: ClientOptions = ClientOptions::parse(get_env("DB_CONN_STRING")).await?;

    client_options.app_name = Some("Rust MongoDB Backup".to_string());

    let client: Client = Client::with_options(client_options)?;

    println!("Trying to connect");
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Connected successfully.");

    let db: Database = client.database(&get_env("DB_NAME").as_str());

    let collection: Collection<Book> = db.collection::<Book>("books");

    let docs: Vec<Book> = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "Animal Farm".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ];

    collection.insert_many(docs, None).await?;

    Ok(())
}

pub fn get_env(variable: &str) -> String {
    let error_message: String = format!("{} must be set.", variable);
    return std::env::var(variable).expect(error_message.as_str());
}
