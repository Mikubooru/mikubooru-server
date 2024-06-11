use sqlx::{FromRow, Type};

#[derive(Type, Debug, Default)]
#[sqlx(transparent)]
pub struct UserId(pub u32);

#[derive(Type, Debug, Default)]
#[sqlx(transparent)]
pub struct UserVocaDBId(pub u32);

#[derive(Type, Debug)]
pub enum UserRole {
    Default,
    Trusted,
    Moderator,
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Default
    }
}

pub struct NewUser {
    pub name: String,
    pub role: UserRole,
    pub vocadb_id: UserVocaDBId,
}

#[derive(FromRow, Debug, Default)]
#[sqlx(rename_all = "camelCase")]
pub struct User {
    id: UserId,
    name: String,
    role: UserRole,
    vocadb_id: UserVocaDBId,
}
