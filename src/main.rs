use actix_web::{web, App, HttpServer};

use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use steam_server_manager2::config;

mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("couldnt build db pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
