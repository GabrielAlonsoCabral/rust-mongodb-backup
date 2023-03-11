use chrono::{self};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use models::{get_env, Point};
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

    let collection: Collection<Point> = db.collection::<Point>("points");

    let mut cursor: Cursor<Point> = collection.find(None, None).await?;

    let mut string_contents: Vec<String> = vec![];

    while let Some(point) = cursor.try_next().await? {
        let string_content: String = format!(
            "`{{`
            '_id':'{}',
            'id':'{:?}',
            'location':'{:?}',
            'attributes':'{:?}',
            'layer':'{}',
            'user':{:?}
        `}}`",
            point._id,
            point.id.ok_or('0'),
            point.location.unwrap(),
            point.attributes.ok_or("[]"),
            point.layer,
            point.user,
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

    let filename: String = format!("./backups/backup-points-{}.json", current_date);

    let mut file: File = File::create(filename.as_str()).unwrap();
    file.write_all(formatted_content.as_bytes()).unwrap();

    Ok(())
}
