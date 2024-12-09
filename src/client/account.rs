use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::doc;
use mongodb::Client;
use crate::structs::account::Account;

pub async fn check_user_exists(client: &Client, email: &str) -> bool {
    let db = client.database("rms");
    let collection = db.collection::<Account>("users");
    let user = collection.find_one(doc! { "email": email }).await;
    match user {
        Ok(Some(_)) => true,
        _ => false,
    }
}

pub async fn check_account(email: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let user_exists = check_user_exists(&client, &email).await;
    if user_exists {
        HttpResponse::Ok().json(format!("User with email {} exists.", email))
    } else {
        HttpResponse::NotFound().json(format!("User with email {} not found.", email))
    }
}

pub async fn create_account(user: web::Json<Account>, client: web::Data<Client>) -> HttpResponse {
    let db = client.database("rms");
    let collection = db.collection("users");
    let doc = collection.find_one(doc! { "email": &user.email });
    if let Ok(Some(_)) = doc.await {
        return HttpResponse::Conflict().body("User already exists.")
    }

    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let user_doc = doc! {
        "email": &user.email,
        "password": hashed_password,
        "username": &user.username,
    };

    match collection.insert_one(user_doc).await {
        Ok(_) => {
            HttpResponse::Created().body("User account created.")
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Failed to create account.")
        }
    }

}