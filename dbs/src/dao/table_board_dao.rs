use common::global::Config;
use sea_orm::*;
use sqlx::types::chrono::Utc;
use crate::model::table_board::{Entity, Model};

pub struct TableBoardDao;
impl TableBoardDao {
    pub async fn insert(model: Model) -> Result<Model, DbErr> {
        let mut active_model = model.into_active_model();
        active_model.id = NotSet;
        active_model.created_at = ActiveValue::Set(Option::from(Utc::now()));
        active_model.save(Config::get_db()).await?.try_into_model()
    }

    pub async fn delete(model: Model) -> Result<DeleteResult, DbErr> {
        let active_model = model.into_active_model();
        active_model.delete(Config::get_db()).await
    }

    pub async fn update(model: Model) -> Result<Model, DbErr> {
        let mut active_model = model.into_active_model();
        active_model.updated_at = ActiveValue::set(Option::from(Utc::now()));
        active_model
            .update(Config::get_db())
            .await?
            .try_into_model()
    }

    pub async fn find_by_id(model: Model) -> Result<Option<Model>, DbErr> {
        let result = Entity::find_by_id(model.id).one(Config::get_db()).await;
        result
    }

    pub async fn find_one(condition: Condition) -> Result<Option<Model>, DbErr> {
        let result = Entity::find().one(Config::get_db()).await;
        result
    }

    pub async fn find_list(condition: Condition) -> Result<Vec<Model>, DbErr> {
        let list = Entity::find().filter(condition).all(Config::get_db()).await;
        list
    }

    pub async fn find_page(
        condition: Condition,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<Model>, DbErr> {
        let paginator = Entity::find()
            .filter(condition)
            .paginate(Config::get_db(), page_size);
        paginator.fetch_page(page).await
    }
}
