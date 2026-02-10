pub mod user_repo;

use mongodb::{Client, Database, options::ClientOptions};

pub async fn establish_connection(db_url: &str, db_name: &str) -> Database {
    // 1. Parse connection string
    let mut client_options = ClientOptions::parse(db_url)
        .await
        .expect("Failed to parse MongoDB URI");

    // 2. Enterprise Tip: Set pool size and timeouts
    client_options.app_name = Some("NullTalkAPI".to_string());
    client_options.max_pool_size = Some(50); // Adjust based on your 100k connection testing!

    // 3. Create Client
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");

    // 4. Ping the database to ensure it's alive before starting the server
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
        .expect("Could not connect to MongoDB");

    client.database(db_name)
}
