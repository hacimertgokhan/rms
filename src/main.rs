use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions, bson::doc };
use serde::{Deserialize, Serialize};
use std::env;
use crate::client::account::{check_account, create_account, get_exists_user};

mod cli;
mod jwt;

mod client {
    pub mod account;
}

mod structs {
    pub mod account;
}

async fn connect_to_mongo() -> mongodb::error::Result<Client> {
    dotenv::dotenv().ok();
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mut client_options = ClientOptions::parse(&uri).await?;
    client_options.app_name = Some("ActixWebMongoDB".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_to_mongo().await.expect("Failed to connect to MongoDB");
    println!("Server started at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/account/check/{email}", web::get().to(check_account))
            .route("/account/create", web::post().to(create_account))
            .route("/account/user", web::get().to(get_exists_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
