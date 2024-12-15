use crate::structs::account::Account;
use crate::client::account::__path_check_account;
use crate::client::account::__path_get_exists_user;
use crate::client::account::__path_create_account;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions, bson::doc };
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::{oauth, Config, SwaggerUi};
use log::{info, LevelFilter};
use env_logger::Builder;
use crate::client::account::{check_account, create_account, get_exists_user};
mod cli;
mod jwt;

mod client {
    pub mod account;
}

mod sectionizer {
    pub mod sections;
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


#[derive(OpenApi)]
#[openapi(paths(
    create_account,
    get_exists_user,
    check_account),
    components(schemas(Account)),
    tags(
        (name = "Accounts", description = "Account management endpoints"),
        (name = "Sectionizer", description = "Sectionizer management endpoints")
    )
)]
struct ApiDoc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_to_mongo().await.expect("Failed to connect to MongoDB");
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    let host = "127.0.0.1";
    let port = 8080;
    info!("🧭 Server started on {}:{}", host, port);
    info!("🚀 Swagger UI is available at: http://{}:{}/swagger-ui/index.html", host, port);
    info!("📚 OpenAPI Documentation available at: http://{}:{}/api-docs/openapi.json", host, port);

    let swagger = SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .config(Config::default()
            .try_it_out_enabled(true)
            .filter(true)
        )
        .oauth(oauth::Config::new());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/account/check/{email}", web::get().to(check_account))
            .route("/account/create", web::post().to(create_account))
            .route("/account/user", web::get().to(get_exists_user))
            .service(swagger.clone())
    })
        .bind((host, port))?
        .run()
        .await
}
