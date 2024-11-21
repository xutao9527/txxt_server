use dbs::migrate;
use sea_orm::DatabaseBackend;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    common::global::Config::initialize().await;
    migrate::start_migrate(DatabaseBackend::Sqlite).await;
}
