use chrono::{self};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    _id: ObjectId,
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let mut client_options: ClientOptions = ClientOptions::parse(get_env("DB_CONN_STRING")).await?;

    client_options.app_name = Some("Rust MongoDB Collection Backup".to_string());

    let client: Client = Client::with_options(client_options)?;

    println!("Trying to connect");

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Connected successfully.");

    let db: Database = client.database(&get_env("DB_NAME"));

    let collection: Collection<Book> = db.collection::<Book>("books");

    let mut cursor: Cursor<Book> = collection.find(None, None).await?;

    let mut string_contents: Vec<String> = vec![];

    while let Some(book) = cursor.try_next().await? {
        let string_content: String = format!(
            "`{{`
            '_id':'{}',
            'title':'{}',
            'author':'{}'
        `}}`",
            book._id, book.title, book.author
        )
        .replace("`", "")
        .replace("'", "\\\"")
        .replace("\\", "");

        string_contents.push(string_content);
    }

    let formatted_content: String = format!("[{}]", string_contents.join(", "));
    let current_date: String = chrono::offset::Utc::now()
        .format("%d-%m-%Y %H:%M:%S")
        .to_string();

    let filename: String = format!("./backups/backup-book-{}.json", current_date);

    let mut file: File = File::create(filename.as_str()).unwrap();
    file.write_all(formatted_content.as_bytes()).unwrap();

    Ok(())
}

pub fn get_env(variable: &str) -> String {
    let error_message: String = format!("{} must be set.", variable);
    return std::env::var(variable).expect(error_message.as_str());
}
