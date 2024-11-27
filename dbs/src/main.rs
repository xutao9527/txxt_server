use dbs::migrate;
use sea_orm::{ColumnTrait, Condition, DatabaseBackend};
use sea_orm::sea_query::{ExprTrait, Query};

use dbs::dao::TableBoardDao;
use dbs::model::table_board;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    common::global::Config::initialize().await;
    //migrate::start_migrate(DatabaseBackend::Sqlite).await;
    let condition = Condition::all().add(table_board::Column::TableNo.eq("101")).add(table_board::Column::Password.eq("7735a36e802561ef44249c039c8db410"));
    let expr = table_board::Column::TableNo.eq("101");
    let result1 = TableBoardDao::find_one(condition).await;
    let result2 = TableBoardDao::find_one(table_board::Column::TableNo.eq("101")).await;
    println!("{:?}",result1);
    println!("{:?}",result2);
}
