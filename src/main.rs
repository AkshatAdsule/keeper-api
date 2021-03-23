use actix_cors::Cors;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use keeper_api::db::DBService;
use keeper_api::Note;
use std::sync::Mutex;

#[get("/")]
async fn index(data: web::Data<Mutex<DBService>>) -> impl Responder {
    let db =data.lock().unwrap();
    println!("{:?}", db.get_all().await.unwrap());
    HttpResponse::Ok().json(db.get_all().await.unwrap())
}

#[post("/")]
async fn post(note: web::Json<Note>, data: web::Data<Mutex<DBService>>) -> impl Responder {
    let db = data.lock().unwrap();
    db.create(&note).await.unwrap();
    println!("{:?}", note);
    HttpResponse::Ok()
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>, data: web::Data<Mutex<DBService>>) -> impl Responder {
    let db = data.lock().unwrap();
    db.delete(id.as_str()).await.unwrap();
    println!("Got id: {}", id);
    HttpResponse::Ok()
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
