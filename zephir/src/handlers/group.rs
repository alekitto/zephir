use crate::err::ZephirError;
use crate::handlers::policy::UpsertPolicyRequest;
use actix_web::{get, patch, post, web, HttpResponse};
use actix_web_validator::Validate;
use libzephir::identity::group::Group;
use libzephir::policy::policy::{CompletePolicy, ToJson};
use libzephir::policy::policy_set::PolicySetTrait;
use libzephir::storage::StorageManager;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct UpsertGroupRequest {
    #[validate(length(min = 1, message = "The value is too short"))]
    id: String,
    linked_policies: Vec<String>,
    #[validate]
    inline_policy: Option<UpsertPolicyRequest>,
}

type StringType<'a> = &'a str;
#[derive(Debug)]
enum PatchOperation {
    Add,
    Remove,
}

impl Deserialize<'de> for PatchOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let str = StringType::deserialize(deserializer)?;
        match str {
            "add" => Ok(PatchOperation::Add),
            "remove" => Ok(PatchOperation::Remove),
            _ => Err(serde::de::Error::invalid_value(
                Unexpected::Str(str),
                &r#""add" or "remove""#,
            )),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct PatchGroupIdentitiesRequest {
    operation: PatchOperation,
    identity: String,
}

#[post("/groups")]
pub(crate) async fn upsert_group(
    info: web::Json<UpsertGroupRequest>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    info.validate()?;
    let inline_policy = match info.0.inline_policy {
        Option::None => Option::None,
        Option::Some(req_policy) => Option::Some(CompletePolicy::try_from(req_policy)?),
    };

    let mut group = Group::new(info.0.id, inline_policy);
    for ref p in info.0.linked_policies {
        match storage.find_policy(p).await? {
            Option::None => {
                return Ok(HttpResponse::BadRequest().json(format!("Policy {} does not exist", p)))
            }
            Option::Some(policy) => group = group.add_policy(policy),
        };
    }

    storage.save_group(&group).await?;
    Ok(HttpResponse::Ok().json(group.to_json()))
}

#[get("/group/{id}")]
pub(crate) async fn get_group(
    web::Path(id): web::Path<String>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    let result = storage.find_group(id).await?;
    match result {
        Option::None => Err(ZephirError::NotFound),
        Option::Some(group) => Ok(HttpResponse::Ok().json(group.to_json())),
    }
}

#[get("/group/{id}/identities")]
pub(crate) async fn get_group_identities(
    web::Path(id): web::Path<String>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    let result = storage.find_group(id).await?;
    match result {
        Option::None => Err(ZephirError::NotFound),
        Option::Some(group) => Ok(HttpResponse::Ok().json(
            group
                .get_identities()
                .await
                .into_iter()
                .map(|i| i.get_id())
                .collect::<Vec<&String>>(),
        )),
    }
}

#[patch("/group/{id}/identities")]
pub(crate) async fn patch_group_identities(
    info: web::Json<PatchGroupIdentitiesRequest>,
    web::Path(id): web::Path<String>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    let result = storage.find_group(id).await?;
    match result {
        Option::None => Err(ZephirError::NotFound),
        Option::Some(mut group) => {
            match info.operation {
                PatchOperation::Add => match storage.find_identity(&info.identity).await? {
                    Option::None => Err(ZephirError::NotFound),
                    Option::Some(identity) => {
                        group = group.add_identity(identity);
                        Ok(storage.save_group(&group).await?)
                    }
                },
                PatchOperation::Remove => {
                    group = group.remove_identity(&info.identity);
                    Ok(storage.save_group(&group).await?)
                }
            }?;

            Ok(HttpResponse::NoContent().finish())
        }
    }
}
