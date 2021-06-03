mod group_manager;
mod identity_manager;
mod policy_manager;
mod types;

use sqlx::{Any, Pool};

#[derive(Clone)]
pub struct StorageManager {
    pool: Pool<Any>,
}

impl StorageManager {
    pub fn new(pool: Pool<Any>) -> Self {
        StorageManager { pool }
    }
}
