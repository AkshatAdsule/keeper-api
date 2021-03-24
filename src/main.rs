use actix_cors::Cors;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use keeper_api::db::DBService;
use keeper_api::Note;
use std::sync::Mutex;

#[get("/")]
async fn index(data: web::Data<Mutex<DBService>>) -> impl Responder {
    let db =data.lock().unwrap();
    let notes = match db.get_all().await {
        Ok(notes) => notes,
        Err(_) => vec![]
    };
    HttpResponse::Ok().json(notes)
}

#[post("/")]
async fn post(note: web::Json<Note>, data: web::Data<Mutex<DBService>>) -> impl Responder {
    let db = data.lock().unwrap();
    match db.create(&note).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>, data: web::Data<Mutex<DBService>>) -> impl Responder {
    let db = data.lock().unwrap();
    match db.delete(&id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(Mutex::new(DBService::new().await.unwrap()));
    HttpServer::new(move|| {
        App::new()
            .app_data(client.clone())
            .wrap(Cors::new().allowed_origin("http://localhost:3000").finish())
            .service(index)
            .service(post)
            .service(delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
