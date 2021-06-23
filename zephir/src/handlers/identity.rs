use crate::err::ZephirError;
use crate::handlers::policy::InlinePolicy;
use actix_web::{get, post, web, HttpResponse};
use actix_web_validator::Validate;
use libzephir::identity::identity::Identity;
use libzephir::policy::policy::{CompletePolicy, ToJson};
use libzephir::policy::policy_set::PolicySetTrait;
use libzephir::storage::StorageManager;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct UpsertIdentityRequest {
    #[validate(length(min = 1, message = "The value is too short"))]
    id: String,
    linked_policies: Vec<String>,
    #[validate]
    inline_policy: Option<InlinePolicy>,
}

#[post("/identities")]
pub(crate) async fn upsert_identity(
    info: web::Json<UpsertIdentityRequest>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    info.validate()?;
    let inline_policy = match info.0.inline_policy {
        Option::None => Option::None,
        Option::Some(req_policy) => Option::Some(CompletePolicy::try_from(req_policy)?),
    };

    let mut identity = Identity::new(info.0.id, inline_policy);
    for ref p in info.0.linked_policies {
        match storage.find_policy(p).await? {
            Option::None => {
                return Ok(HttpResponse::BadRequest().json(format!("Policy {} does not exist", p)))
            }
            Option::Some(policy) => identity = identity.add_policy(policy),
        };
    }

    storage.save_identity(&identity).await?;
    Ok(HttpResponse::Ok().json(identity.to_json()))
}

#[get("/identity/{id}")]
pub(crate) async fn get_identity(
    web::Path(id): web::Path<String>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    let result = storage.find_identity(id).await?;
    match result {
        Option::None => Err(ZephirError::NotFound),
        Option::Some(identity) => Ok(HttpResponse::Ok().json(identity.to_json())),
    }
}
