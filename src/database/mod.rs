use mongodb::{Client, Database};

pub async fn connect(
    mongodb_uri: &str,
    database_name: &str,
) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(mongodb_uri).await?;

    let database = client.database(database_name);

    database.run_command(mongodb::bson::doc! { "ping": 1 }).await?;

    println!("successfully connected to MongoDB");

    Ok(database)
}