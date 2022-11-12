use actix_web::{delete, web};
use actix_web::{get, post};
use actix_web::{Error, HttpResponse};
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;

mod db;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/server")]
pub async fn servers(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let servers = web::block(move || {
        let mut conn = pool.get()?;
        db::models::Server::list(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match servers {
        Ok(servers) => Ok(HttpResponse::Ok().json(servers)),
        Err(e) => {
            let res = HttpResponse::InternalServerError()
                .body(format!("issue retrieving server list: {}", e));
            Ok(res)
        }
    }
}

#[post("/server")]
pub async fn add_server(
    pool: web::Data<DbPool>,
    data: web::Json<db::models::Server>,
) -> Result<HttpResponse, Error> {
    let server = web::block(move || {
        let mut conn = pool.get()?;
        db::models::Server::save(&mut conn, &data.0)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match server {
        Ok(_) => Ok(HttpResponse::Created().body("")),
        Err(e) => {
            let res = HttpResponse::InternalServerError().body(format!("{}", e));
            Ok(res)
        }
    }
}

#[get("/server/{id}")]
pub async fn get_server(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let server = web::block(move || {
        let mut conn = pool.get()?;
        db::models::Server::get(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match server {
        Ok(server) => Ok(HttpResponse::Ok().json(server)),
        Err(e) => {
            let res = HttpResponse::InternalServerError().body(format!("{}", e));
            Ok(res)
        }
    }
}

#[delete("/server/{id}")]
pub async fn delete_server(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let server = web::block(move || {
        let mut conn = pool.get()?;
        db::models::Server::delete(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match server {
        Ok(_) => Ok(HttpResponse::Ok().body("")),
        Err(e) => {
            let res = HttpResponse::InternalServerError().body(format!("{}", e));
            Ok(res)
        }
    }
}
