use actix_web::{web, HttpResponse, Responder};
use actix_web::web::delete;
use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::doc;
use mongodb::Client;
use uuid::Uuid;
use crate::structs::account::{Account, AccountRequestByEmail, AccountRequestByIDRequest, User};

pub async fn check_user_exists(client: &Client, email: &str) -> bool {
    let db = client.database("rms");
    let collection = db.collection::<Account>("users");
    let user = collection.find_one(doc! { "email": email }).await;
    match user {
        Ok(Some(_)) => true,
        _ => false,
    }
}

#[utoipa::path(
    get,
    tag = "Accounts",
    path = "/account/check/{email}",
    request_body = AccountRequestByEmail,
    responses(
        (status = 201, description = "User account created"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn check_account(email: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let user_exists = check_user_exists(&client, &email).await;
    if user_exists {
        HttpResponse::Ok().json(format!("User with email {} exists.", email))
    } else {
        HttpResponse::NotFound().json(format!("User with email {} not found.", email))
    }
}

#[utoipa::path(
    get,
    tag = "Accounts",
    path = "/account/user",
    request_body = AccountRequestByIDRequest,
    responses(
        (status = 201, description = "User account created"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_exists_user(user: String, client: web::Data<Client>) -> HttpResponse {
    let db = client.database("rms");
    let collection = db.collection::<User>("users");
    let doc = collection.find_one(doc! { "id": &user }).await;
    match doc {
        Ok(Some(_)) => HttpResponse::Ok().json(&user.to_string()),
        Ok(None) => {
            HttpResponse::Conflict().body("User already exists.")
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to query database."),
    }
}

#[utoipa::path(
    post,
    tag = "Accounts",
    path = "/account/create",
    request_body = Account,
    responses(
        (status = 201, description = "User account created"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_account(user: web::Json<Account>, client: web::Data<Client>) -> HttpResponse {
    let db = client.database("rms");
    let collection = db.collection("users");
    if let Ok(Some(_)) = collection.find_one(doc! { "email": &user.email }).await {
        return HttpResponse::Conflict().body("User already exists.");
    }
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };
    let uuid = Uuid::new_v4();
    let user_doc = doc! {
        "email": &user.email,
        "password": hashed_password,
        "username": &user.username,
        "admin": &user.admin,
        "id": uuid.to_string(),
    };
    match collection.insert_one(user_doc).await {
        Ok(_) => HttpResponse::Created().body("User account created."),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create account."),
    }
}
