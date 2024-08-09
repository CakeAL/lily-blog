use std::env;

use anyhow::Result;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub enum Service {
    Tag,
    Post,
    Comment,
    Admin,
}

pub async fn get_db_connection() -> Result<DatabaseConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub fn get_service_addr(srv: Service) -> Result<String> {
    dotenv().ok();
    match srv {
        Service::Tag => Ok(format!("[::1]:{}", env::var("TAG_SRV_PORT")?)),
        Service::Post => Ok(format!("[::1]:{}", env::var("POST_SRV_PORT")?)),
        Service::Comment => Ok(format!("[::1]:{}", env::var("COMMENT_SRV_PORT")?)),
        Service::Admin => Ok(format!("[::1]:{}", env::var("ADMIN_SRV_PORT")?)),
    }
}

pub fn get_service_url(srv: Service) -> Result<String> {
    Ok(format!("http://{}", get_service_addr(srv)?))
}