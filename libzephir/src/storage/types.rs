use sqlx::types::Json;

#[derive(sqlx::FromRow)]
pub(super) struct DbIdentity {
    pub(super) id: String,
    pub(super) policy_id: Option<String>,
}

#[derive(sqlx::Type, sqlx::FromRow)]
pub(super) struct DbPolicy {
    pub(super) id: String,
    pub(super) version: i32,
    pub(super) effect: bool,
    pub(super) actions: Json<Vec<String>>,
    pub(super) resources: Json<Vec<String>>,
}
