use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::OnceLock};

pub static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub db: Option<DatabaseConnection>,
    pub app_name: Option<String>,
}

impl Config {
    // 异步初始化方法
    pub async fn initialize() {
        GLOBAL_CONFIG.get_or_init(|| {
            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
            let db_connection = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(Database::connect(db_url))
            })
            .expect("Failed to connect to database");
            Config {
                db: Some(db_connection),
                app_name: Some("MyApp".to_string()),
            }
        });
    }

    // 获取全局配置
    pub fn global() -> &'static Config {
        GLOBAL_CONFIG
            .get()
            .expect("GLOBAL_CONFIG is not initialized")
    }

    // 获取 DatabaseConnection
    pub fn get_db() -> &'static DatabaseConnection {
        Self::global()
            .db
            .as_ref()
            .expect("DatabaseConnection is not initialized")
    }
}
