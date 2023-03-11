use chrono::{self};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use models::{get_env, User};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Cursor, Database};
use std::{fs::File, io::Write};

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

    let collection: Collection<User> = db.collection::<User>("users");

    let mut cursor: Cursor<User> = collection.find(None, None).await?;

    let mut string_contents: Vec<String> = vec![];

    while let Some(user) = cursor.try_next().await? {
        let string_content: String = format!(
            "`{{`
            '_id':'{}',
            'email':'{}',
            'name':'{}',
            'password':'{}',
            'type':'{}',
            'permissions':{:?},
            'blocked':{},
            'company':'{}',
            'status':'{}',
            'deleted':'{:?}'
        `}}`",
            user._id,
            user.email,
            user.name,
            user.password,
            user.kind,
            user.permissions,
            user.blocked,
            user.company,
            user.status,
            user.deleted
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

    let filename: String = format!("./backups/backup-users-{}.json", current_date);

    let mut file: File = File::create(filename.as_str()).unwrap();
    file.write_all(formatted_content.as_bytes()).unwrap();

    Ok(())
}
