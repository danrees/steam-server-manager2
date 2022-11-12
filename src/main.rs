use actix_web::{App, HttpServer};
use steam_server_manager2::servers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(servers))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;

    Ok(())
}
