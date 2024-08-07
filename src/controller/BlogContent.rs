use actix_web::{get, put, post, delete, web, HttpResponse};
use futures::stream::StreamExt;
use mongodb::{bson::{doc, oid::ObjectId, to_bson}, Client, Collection};

use crate::model::BlogContent;

const DB_NAME: &str = "lucy-hobby-blog-db";
const COLL_NAME: &str = "debug-collection";

#[post("/")]
pub async fn add_blog_content(client: web::Data<Client>, form: web::Json<BlogContent>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("BlogContent added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/")]
async fn get_all_blog_content(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<BlogContent> = client.database(DB_NAME).collection(COLL_NAME);
    let mut cursor = collection.find(None, None).await.expect("Error: not being able to get data from database");

    let mut results: Vec<BlogContent> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => results.push(document),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
        }
    }

    HttpResponse::Ok().json(results)
}

#[get("/{id}")]
async fn get_blog_content(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let collection: Collection<BlogContent> = client.database(DB_NAME).collection(COLL_NAME);
    let id = id.into_inner();
    let object_id = ObjectId::parse_str(&id).expect("invalid id");
    let result = collection.find_one(doc! {"_id": &object_id}, None).await;

    match result {
        Ok(Some(blog_content)) => HttpResponse::Ok().json(blog_content),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No blog_content found with id {id}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
