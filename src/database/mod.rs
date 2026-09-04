// GRABING TOOLS FROM MONGODB; 
// CLIENT THIS IS THE MEDIUM IN WHICH THE DATABASE AND THE SERVER CONNECT
// DATABASE REPRESENT THE PARTICULAR MONGODB DATABASE WE ARE WORKING WITH
use mongodb::{Client, Database};

// CREATE A FUNCTION CALLED CONNECT
// THIS RECEIVE THE MONGODB ADDRESS AND DATABASE NAME
// CONNECT TO MONGODB AND GIVES US THE DATABASE
pub async fn connect(
    mongodb_uri: &str, //USING THIS VALLUE FROM MOD.RS/CONFIG
    database_name: &str, //USING THIS VALLUE FROM MOD.RS/CONFIG
) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(mongodb_uri).await?; //WE CREATE CLIENT WITH THIS

    let database = client.database(database_name); //TELLING THE MONGODB SERVER I WANT TO WORK WITH THIS PARTICULAR DATABASE NAME

    database.run_command(mongodb::bson::doc! { "ping": 1 }).await?; //THIS IS USED TO TEST THE MONGODB CONNECTION

    tracing::info!("successfully connected to MongoDB"); 
    Ok(database)
}