use crate::err::Error;
use crate::identity::group::Group;
use crate::identity::identity::Identity;
use crate::identity::role::Role;
use crate::identity::subject::Subject;
use crate::policy::policy::CompletePolicy;
use crate::policy::policy_set::PolicySetTrait;
use crate::storage::types::{DbIdentity, DbPolicy};
use crate::storage::StorageManager;
use std::convert::TryFrom;

impl StorageManager {
    pub async fn find_groups_for_identity(&self, target: &Identity) -> Result<Vec<Group>, Error> {
        let groups = sqlx::query_as::<_, DbIdentity>(r#"
            SELECT id, policy_id
            FROM "group"
            INNER JOIN group_identity ON "group".id = group_identity.group_id AND group_identity.identity_id = $1
        "#)
            .bind(&target.id)
            .fetch_all(&self.pool)
            .await?;

        let mut result = vec![];
        for g in groups {
            result.push(self._load_group(&g).await?);
        }

        Ok(result)
    }

    pub async fn find_group<S>(&self, id: S) -> Result<Option<Group>, Error>
    where
        S: ToString,
    {
        let group = sqlx::query_as::<_, DbIdentity>(
            r#"
            SELECT id, policy_id
            FROM group
            WHERE id = $1
        "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if group.is_none() {
            return Ok(Option::None);
        }

        Ok(Some(self._load_group(group.as_ref().unwrap()).await?))
    }

    async fn _load_group(&self, group: &DbIdentity) -> Result<Group, Error> {
        let inline_policy = if group.policy_id.is_some() {
            self.find_policy(group.policy_id.as_ref().unwrap()).await?
        } else {
            Option::None
        };

        let mut group = Group::new(group.id.to_string(), inline_policy);
        let policies = sqlx::query_as::<_, DbPolicy>(
            r#"
            SELECT id, version, effect, actions, resources
            FROM policy
            INNER JOIN group_policy ip ON ip.policy_id = policy.id AND ip.group_id = $1
        "#,
        )
        .bind(&group.name)
        .fetch_all(&self.pool)
        .await?;

        for db_policy in policies {
            group = group.add_policy(CompletePolicy::try_from(db_policy)?);
        }

        let identities: Vec<DbIdentity> = sqlx::query_as::<_, DbIdentity>(
            r#"
            SELECT id, policy_id
            FROM identity
            INNER JOIN group_identity gi ON gi.identity_id = identity.id AND gi.group_id = $1
        "#,
        )
        .bind(&group.name)
        .fetch_all(&self.pool)
        .await?;

        for i in identities {
            group = group.add_identity(self.find_identity(i.id).await?.unwrap());
        }

        Ok(group)
    }

    pub async fn save_group(&self, g: &Group) -> Result<(), Error> {
        let embedded_policy = g.get_inline_policy();

        let mut transaction = self.pool.begin().await?;
        if embedded_policy.is_some() {
            self._save_policy(embedded_policy.unwrap(), &mut transaction)
                .await?;
        } else {
            let policy_id = "__embedded_policy_group_".to_owned() + g.name.as_str() + "__";
            sqlx::query("DELETE FROM policy WHERE id = $1")
                .bind(policy_id)
                .execute(&mut transaction)
                .await?;
        }

        sqlx::query(
            r#"
            INSERT INTO group(id, policy_id)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET policy_id = $2
        "#,
        )
        .bind(&g.name)
        .bind(if embedded_policy.is_none() {
            Option::None
        } else {
            Option::Some(&embedded_policy.unwrap().id)
        })
        .execute(&mut transaction)
        .await?;

        sqlx::query("DELETE FROM group_policy WHERE group_id = $1")
            .bind(&g.name)
            .execute(&mut transaction)
            .await?;

        for p in g.linked_policies() {
            sqlx::query(
                r#"
                INSERT INTO group_policy (group_id, policy_id)
                VALUES ($1, $2)
            "#,
            )
            .bind(&g.name)
            .bind(&p.id)
            .execute(&mut transaction)
            .await?;
        }

        sqlx::query("DELETE FROM group_identity WHERE group_id = $1")
            .bind(&g.name)
            .execute(&mut transaction)
            .await?;

        for i in &g.identities {
            sqlx::query(
                r#"
                INSERT INTO group_identity (group_id, identity_id)
                VALUES ($1, $2)
            "#,
            )
            .bind(&g.name)
            .bind(&i.id)
            .execute(&mut transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
