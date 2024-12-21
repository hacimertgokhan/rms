use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::Client;
use uuid::Uuid;
use crate::structs::account::{User};
use crate::structs::sections::{Sections, SectionsByID};

#[utoipa::path(
    get,
    tag = "Sections",
    path = "/sections/get/{id}",
    request_body = SectionsByID,
    responses(
        (status = 201, description = "Section found."),
        (status = 409, description = "Section cannot found."),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_section_by_id(table: String, client: web::Data<Client>) -> HttpResponse {
    let db = client.database("rms");
    let collection = db.collection::<User>("sections");
    let doc = collection.find_one(doc! { "id": &table }).await;
    match doc {
        Ok(Some(_)) => HttpResponse::Ok().json(&table.to_string()),
        Ok(None) => {
            HttpResponse::Conflict().body("Section not found.")
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to query database."),
    }
}

#[utoipa::path(
    post,
    tag = "Sections",
    path = "/sections/create",
    request_body = Sections,
    responses(
        (status = 201, description = "Section created"),
        (status = 409, description = "Section already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_section(sections: web::Json<Sections>, client: web::Data<Client>) -> HttpResponse {
    let db = client.database("rms");
    let collection = db.collection("sections");
    if let Ok(Some(_)) = collection.find_one(doc! { "id": &sections.id }).await {
        return HttpResponse::Conflict().body("Section already exists.");
    }
    let uuid = Uuid::new_v4();
    let user_doc = doc! {
        "id": &sections.id,
        "title": &sections.title,
        "subtitle": &sections.subtitle,
        "uuid": uuid.to_string(),
        "full": false,
        "total": 0,
    };
    match collection.insert_one(user_doc).await {
        Ok(_) => HttpResponse::Created().body("Section created."),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create section."),
    }
}
