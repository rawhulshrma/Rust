use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}

struct AppState {
    items: Mutex<HashMap<u32, Item>>,
}

async fn create_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.insert(item.id, item.into_inner());
    HttpResponse::Created().finish()
}

async fn read_item(web::Path(id): web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    if let Some(item) = items.get(&id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_item(id: web::Path<u32>, item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if items.contains_key(&id) {
        items.insert(*id, item.into_inner());
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_item(web::Path(id): web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if items.remove(&id).is_some() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn list_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    let items: Vec<Item> = items.values().cloned().collect();
    HttpResponse::Ok().json(items)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/items", web::post().to(create_item))
            .route("/items", web::get().to(list_items))
            .route("/items/{id}", web::get().to(read_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
