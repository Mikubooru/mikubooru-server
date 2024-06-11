use crate::db::DB;
use crate::db::types::{NewUser, User};

pub async fn add_user(db: &DB, new_user: NewUser) -> Result<User, sqlx::Error> {
    let res: Result<User, _> =
        sqlx::query_as("insert into user (name, role, vocadbId) values ($1, $2, $3) returning *")
            .bind(&new_user.name)
            .bind(&new_user.role)
            .bind(&new_user.vocadb_id)
            .fetch_one(db)
            .await;

    res
}

// pub async fn login(db: &DB, )
