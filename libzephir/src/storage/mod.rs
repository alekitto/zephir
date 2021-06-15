mod group_manager;
mod identity_manager;
mod policy_manager;
mod types;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct StorageManager {
    pool: Pool<Postgres>,
}

impl StorageManager {
    pub fn new(pool: Pool<Postgres>) -> Self {
        StorageManager { pool }
    }
}
