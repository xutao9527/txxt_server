use common::global::Config;
use sea_orm::*;
use sea_query::*;

pub async fn create_table<E>(entity: E, db_backend: DatabaseBackend) -> Result<ExecResult, DbErr>
where
    E: EntityTrait,
{
    let schema = Schema::new(db_backend);
    let mut statement = schema.create_table_from_entity(entity);
    statement.if_not_exists();
    println!("{}", db_backend.build(&statement));
    let result = Config::global()
        .db
        .as_ref()
        .expect("Database connection is not initialized!")
        .execute(db_backend.build(&statement))
        .await;
    result
}

pub async fn insert_data<A, I, C>(models: I, columns: Vec<C>)
where
    C: ColumnTrait,
    A: ActiveModelTrait,
    I: IntoIterator<Item = A>,
{
    type Entity<A> = <A as ActiveModelTrait>::Entity;
    let on_conflict = OnConflict::columns(columns).do_nothing().to_owned();
    let _ = Entity::<A>::insert_many(models)
        .on_conflict(on_conflict)
        .exec(Config::get_db())
        .await;
}
