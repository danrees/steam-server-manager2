use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

#[get("/server")]
pub async fn servers() -> impl Responder {
    HttpResponse::Ok().body("TBD")
}
