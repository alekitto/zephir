use crate::compiler::compiler::cache;
use crate::err::Error;
use crate::policy::policy::{CompletePolicy, MatchablePolicy};
use crate::policy::{PolicyEffect, PolicyVersion};
use crate::storage::types::DbPolicy;
use crate::storage::StorageManager;
use serde_json::Value;
use sqlx::{Any, Transaction};
use std::convert::TryFrom;

impl StorageManager {
    pub async fn find_policy<S>(&self, id: S) -> Result<Option<CompletePolicy>, Error>
    where
        S: ToString,
    {
        let policy = sqlx::query_as::<_, DbPolicy>(
            r#"
            SELECT id, version, effect, actions, resources
            FROM policy
            WHERE id = $1
        "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(match policy {
            Option::None => Option::None,
            Option::Some(policy) => Option::Some(CompletePolicy::try_from(policy)?),
        })
    }

    pub async fn save_policy(&self, p: &CompletePolicy) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        self._save_policy(p, &mut transaction).await?;

        transaction.commit().await?;
        Ok(())
    }

    pub(super) async fn _save_policy(
        &self,
        p: &CompletePolicy,
        transaction: &mut Transaction<'_, Any>,
    ) -> Result<(), Error> {
        let id = p.id.as_str();
        let version: i32 = (&p.version).into();
        let effect: bool = (&p.effect).into();

        sqlx::query(
            r#"
            INSERT INTO policy(id, version, effect, actions, resources)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id)
            DO UPDATE SET version = $2, effect = $3, actions = $4, resources = $5
        "#,
        )
        .bind(id)
        .bind(version)
        .bind(effect)
        .bind(serde_json::to_string(p.get_actions())?)
        .bind(serde_json::to_string(p.get_resources())?)
        .execute(transaction)
        .await?;

        cache::flush_policy(&p.id);
        Ok(())
    }
}

impl TryFrom<DbPolicy> for CompletePolicy {
    type Error = Error;

    fn try_from(value: DbPolicy) -> Result<Self, Self::Error> {
        CompletePolicy::new(
            value.id,
            PolicyVersion::try_from(value.version)?,
            if value.effect {
                PolicyEffect::Allow
            } else {
                PolicyEffect::Deny
            },
            serde_json::from_str::<Vec<String>>(&value.actions)?,
            serde_json::from_str::<Vec<String>>(&value.resources)?,
            Value::Null,
        )
    }
}
