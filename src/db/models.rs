use super::schema::servers;
use super::schema::servers::dsl::servers as servers_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = servers)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub install_dir: String,
}

impl Server {
    pub fn new(id: i32, name: &str, login: &str, install_dir: &str) -> Self {
        Server {
            id,
            name: name.into(),
            login: login.into(),
            install_dir: install_dir.into(),
        }
    }

    pub fn list(conn: &mut SqliteConnection) -> Result<Vec<Self>, anyhow::Error> {
        let result = servers_dsl.load::<Server>(conn)?;
        Ok(result)
    }

    pub fn save(conn: &mut SqliteConnection, server: &Server) -> Result<(), anyhow::Error> {
        diesel::insert_into(servers_dsl)
            .values(server)
            .execute(conn)?;
        Ok(())
    }

    pub fn get(conn: &mut SqliteConnection, server_id: i32) -> Result<Server, anyhow::Error> {
        let server = servers_dsl
            .filter(servers::id.eq(server_id))
            .first::<Server>(conn)?;

        Ok(server)
    }

    pub fn delete(conn: &mut SqliteConnection, server_id: i32) -> Result<(), anyhow::Error> {
        diesel::delete(servers_dsl.filter(servers::id.eq(server_id))).execute(conn)?;
        Ok(())
    }
}
