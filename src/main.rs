use mongodb::{Client, options::ClientOptions};
use std::env;
use std::error::Error;
use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   dotenv().ok();
   let client_options = ClientOptions::parse(env::var("MONGODB_URI").expect("You must set the MONGODB_URI env var!")).await.unwrap();
   let client = Client::with_options(client_options).unwrap();
   println!("{:?}", client);
   // Load the MongoDB connection string from an environment variable:
   // let client_uri =
      // env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   // let options =
     // ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         // .await?;
   // let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   // println!("Databases:");
   // for name in client.list_database_names(None, None).await? {
      // println!("- {}", name);
   // }

   Ok(())
}
