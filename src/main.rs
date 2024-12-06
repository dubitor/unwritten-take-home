use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
enum DummyDataAction {
    Print,
}

#[derive(Deserialize)]
struct DummyDataPayload {
    action: DummyDataAction,
}

#[post("/dummy-data")]
async fn dummy_data(payload: web::Json<DummyDataPayload>) -> impl Responder {
    match payload.into_inner().action {
        DummyDataAction::Print => {
            println!("dummy data");
            return HttpResponse::Ok();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(dummy_data))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
