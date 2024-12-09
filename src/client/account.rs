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