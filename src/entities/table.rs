use log::{error, info};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, Schema};
use crate::entities::prelude::{Categories, CategoryDishMaps, Dishes, Users};

async fn create_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let backend = db_connection.get_database_backend();
    let schema = Schema::new(backend);
    let execution = db_connection.execute(backend.build(&schema.create_table_from_entity(entity)));
    match execution.await {
        Ok(_) => info!("Created {} table.", entity.table_name()),
        Err(e) => {
            error!("create data table error: {}", e);
            panic!("create data table error: {}", e);
        }
    }
}
pub async fn create_all_need_table(db: &DatabaseConnection) {
    create_table(db, Users).await;
    create_table(db, Categories).await;
    create_table(db, Dishes).await;
    create_table(db, CategoryDishMaps).await;
}
#[cfg(test)]
mod test {
    use crate::config::db::{get_db_coon, init_db_coon};
    use crate::entities::table::{create_all_need_table};
    #[tokio::test]
    async fn test_create_table() {
        init_db_coon().await;
        let db = get_db_coon();
        create_all_need_table(&db).await;
        // init_db_coon().await;
        // let db = DB.get().unwrap();
        // // create_table(db, DocAndTags).await;
        // create_table(db, Documents).await;
    }
}
