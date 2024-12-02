pub mod migrate;

use crate::model::table_board;
use sea_orm::{DatabaseBackend, IntoActiveModel};
use sea_orm::sqlx::types::chrono::Utc;

pub async fn start_migrate(db_backend: DatabaseBackend) {
    migrate_table_board(db_backend).await;
}

pub async fn migrate_table_board(db_backend: DatabaseBackend) {
    match migrate::create_table(crate::model::table_board::Entity, db_backend).await {
        Ok(_) => {
            let table_boards = vec![
                table_board::Model {
                    id: 1,
                    table_no: Some("101".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 2,
                    table_no: Some("102".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 3,
                    table_no: Some("103".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 4,
                    table_no: Some("104".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 5,
                    table_no: Some("105".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 6,
                    table_no: Some("106".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 7,
                    table_no: Some("107".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 8,
                    table_no: Some("108".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 9,
                    table_no: Some("109".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 10,
                    table_no: Some("110".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 11,
                    table_no: Some("111".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
                table_board::Model {
                    id: 12,
                    table_no: Some("112".to_string()),
                    password: Some(format!("{:x}", md5::compute("bjl9527"))),
                    status: true,
                    created_at:Some(Utc::now()),
                    updated_at:Some(Utc::now()),
                    ..Default::default()
                },
            ]
                .into_iter()
                .map(|model| model.into_active_model())
                .collect::<Vec<_>>();
            migrate::insert_data(table_boards, vec![table_board::Column::Id]).await;
        }
        Err(_) => {}
    }
}
