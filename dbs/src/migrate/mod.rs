pub mod migrate;
use crate::model::table_board;
use sea_orm::{DatabaseBackend, IntoActiveModel};

pub async fn start_migrate(db_backend: DatabaseBackend) {
    migrate_table_board(db_backend).await;
}

pub async fn migrate_table_board(db_backend: DatabaseBackend) {
    match migrate::create_table(crate::model::table_board::Entity, db_backend).await {
        Ok(_) => {
            let table_boards = vec![
                table_board::Model {
                    id: 1,
                    ..Default::default()
                },
                table_board::Model {
                    id: 2,
                    ..Default::default()
                },
                table_board::Model {
                    id: 3,
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
