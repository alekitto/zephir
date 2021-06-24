use crate::err::Error;
use crate::identity::identity::Identity;
use crate::identity::role::Role;
use crate::policy::policy::CompletePolicy;
use crate::policy::policy_set::PolicySetTrait;
use crate::storage::types::{DbIdentity, DbPolicy};
use crate::storage::StorageManager;
use std::convert::TryFrom;

impl StorageManager {
    pub async fn find_identity<S>(&self, id: S) -> Result<Option<Identity>, Error>
    where
        S: ToString,
    {
        let identity = sqlx::query_as::<_, DbIdentity>(
            r#"
            SELECT id, policy_id
            FROM identity
            WHERE id = $1
        "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if identity.is_none() {
            return Ok(Option::None);
        }

        let identity = identity.unwrap();
        let inline_policy = if identity.policy_id.is_some() {
            self.find_policy(identity.policy_id.unwrap()).await?
        } else {
            Option::None
        };

        let mut identity = Identity::new(identity.id, inline_policy);
        let policies = sqlx::query_as::<_, DbPolicy>(
            r#"
            SELECT id, version, effect, actions, resources
            FROM policy
            INNER JOIN identity_policy ip ON ip.policy_id = policy.id AND ip.identity_id = $1
        "#,
        )
        .bind(id.to_string())
        .fetch_all(&self.pool)
        .await?;

        for db_policy in policies {
            identity = identity.add_policy(CompletePolicy::try_from(db_policy)?);
        }

        Ok(Option::Some(identity))
    }

    pub async fn save_identity(&self, i: &mut Identity) -> Result<(), Error> {
        let embedded_policy = i.inline_policy.as_mut();

        let mut transaction = self.pool.begin().await?;
        let policy_id = format!("__embedded_policy_identity_{}__", i.id);
        let mut policy_param = None;
        if let Some(embedded_policy) = embedded_policy {
            policy_param = Some(policy_id.clone());
            embedded_policy.id = policy_id;
            self._save_policy(embedded_policy, &mut transaction).await?;
        } else {
            sqlx::query("DELETE FROM policy WHERE id = $1")
                .bind(policy_id)
                .execute(&mut transaction)
                .await?;
        }

        sqlx::query(
            r#"
            INSERT INTO identity(id, policy_id)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET policy_id = $2
        "#,
        )
        .bind(&i.id)
        .bind(policy_param)
        .execute(&mut transaction)
        .await?;

        sqlx::query("DELETE FROM identity_policy WHERE identity_id = $1")
            .bind(&i.id)
            .execute(&mut transaction)
            .await?;

        for p in i.linked_policies() {
            sqlx::query(
                r#"
                INSERT INTO identity_policy (identity_id, policy_id)
                VALUES ($1, $2)
            "#,
            )
            .bind(&i.id)
            .bind(&p.id)
            .execute(&mut transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}
