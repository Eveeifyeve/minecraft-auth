use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use std::str;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    code: String,
    state: String,
}

async fn index(info: web::Query<Info>, data: web::Data<Arc<Mutex<Info>>>) -> impl Responder {
    let mut data = data.lock().unwrap();
    data.code = info.code.clone();
    data.state = info.state.clone();
    println!("Access Token: {}", data.code);
    println!("UUID: {}", data.state);
    HttpResponse::Ok().body("❌ If you see this you ran into an Error. Check your console.")
}

#[actix_web::main]
pub async fn main(port: usize, threads: Option<usize>) -> std::io::Result<(String, String)> {
    let shared_data = web::Data::new(Arc::new(Mutex::new(Info {
        code: String::new(),
        state: String::new(),
    })));

    let shared_data_clone = shared_data.clone();
    HttpServer::new(move || App::new().app_data(shared_data_clone.clone()).route("/", web::get().to(index)))
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await?;

    let data = shared_data.into_inner();
    let data = data.lock().unwrap();
    Ok((data.code.clone(), data.state.clone()))
}