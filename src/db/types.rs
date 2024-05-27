use sqlx::{FromRow, Type};

#[derive(Type, Debug)]
#[sqlx(transparent)]
pub struct UserId(u32);

#[derive(Type, Debug)]
#[sqlx(transparent)]
pub struct UserVocaDBId(u32);

#[derive(Type, Debug)]
pub enum UserRole {
    Default,
    Trusted,
    Moderator,
    Admin,
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "camelCase")]
pub struct User {
    id: UserId,
    name: String,
    role: UserRole,
    vocadb_id: UserVocaDBId,
}
