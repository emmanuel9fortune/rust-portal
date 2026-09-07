// GRABING TOOLS FROM MONGODB; 
// CLIENT THIS IS THE MEDIUM IN WHICH THE DATABASE AND THE SERVER CONNECT
// DATABASE REPRESENT THE PARTICULAR MONGODB DATABASE WE ARE WORKING WITH
use mongodb::{
    Client, 
    Database,
    options::IndexOptions,
    bson::doc,
    IndexModel,
};

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

    create_indexes(&database).await?;

    Ok(database)
}

async fn create_indexes(
    database: &Database,
) -> mongodb::error::Result<()> {
    let users = database.collection::<mongodb::bson::Document>("users");

    let email_index = IndexModel::builder()
        .keys(doc! {"email": 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .name("unique_user_email".to_string())
                .build(),
        ).build();

    users.create_index(email_index).await?;

    tracing::info!("Database indexes initialized");

    Ok(())
}