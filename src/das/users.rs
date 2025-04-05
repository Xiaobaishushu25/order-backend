use sea_orm::{ColumnTrait, QueryFilter};
use crate::config::db::get_db_coon;
use crate::entities::prelude::{User, Users};
use crate::error::AppResult;
use sea_orm::{EntityTrait, IntoActiveModel};
use ulid::Ulid;
use crate::entities::users::Column;

pub struct UserCurd;
impl UserCurd {
    /// 插入用户, 返回用户id
    /// 注意这里的密码是经过hash的
    pub async fn insert_user(username: String, password: String) -> AppResult<String> {
        let db = get_db_coon();
        let uuid = Ulid::new();
        let user = User {
            id: uuid.to_string(),
            username,
            password,
        };
        Users::insert(
            user
            .into_active_model(),
        )
        .exec(db)
        .await?;
        Ok(uuid.to_string())
    }
    pub async fn query_by_id(id: String) -> AppResult<Option<User>> {
        let db = get_db_coon();
        let user = Users::find_by_id(id).one(db).await?;
        Ok(user)
    }
    pub async fn query_by_username(name: String) -> AppResult<Option<User>> {
        let db = get_db_coon();
        let user = Users::find()
            .filter(Column::Username.eq(name))
            .one(db)
            .await?;
        Ok(user)
    }
}
