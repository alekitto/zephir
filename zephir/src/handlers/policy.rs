use crate::err::ZephirError;
use actix_web::{get, post, web, HttpResponse};
use actix_web_validator::Validate;
use libzephir::err::Error;
use libzephir::policy::policy::{CompletePolicy, ToJson};
use libzephir::policy::{PolicyEffect, PolicyVersion};
use libzephir::storage::StorageManager;
use regex::Regex;
use serde::Deserialize;
use std::convert::TryFrom;

lazy_static! {
    static ref RE_VALID_ID: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_\-.]*$").unwrap();
    static ref RE_EFFECT: Regex = Regex::new(r"^(ALLOW|DENY)$").unwrap();
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct EmbeddedPolicyRequest {
    #[validate(range(min = 1, max = 1, message = "Invalid version."))]
    version: Option<i32>,
    #[validate(required, regex(path = "RE_EFFECT", message = "Invalid field."))]
    effect: Option<String>,
    #[validate(required, length(min = 1, message = "The value is too short"))]
    actions: Option<Vec<String>>,
    #[validate(length(min = 1, message = "The value is too short"))]
    resources: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct UpsertPolicyRequest {
    #[validate(
        length(min = 1, message = "The value is too short"),
        regex(path = "RE_VALID_ID", message = "Invalid field.")
    )]
    id: String,
    #[validate(range(min = 1, max = 1, message = "Invalid version."))]
    version: i32,
    #[validate(regex(path = "RE_EFFECT", message = "Invalid field."))]
    effect: String,
    #[validate(length(min = 1, message = "The value is too short"))]
    actions: Vec<String>,
    #[validate(length(min = 1, message = "The value is too short"))]
    resources: Option<Vec<String>>,
}

impl TryFrom<EmbeddedPolicyRequest> for CompletePolicy {
    type Error = Error;

    fn try_from(value: EmbeddedPolicyRequest) -> Result<Self, Self::Error> {
        CompletePolicy::new(
            "".to_string(),
            PolicyVersion::try_from(value.version.unwrap_or(1))?,
            PolicyEffect::try_from(&value.effect.unwrap_or_else(|| "ALLOW".to_string()))?,
            value.actions.unwrap_or_default(),
            value.resources.unwrap_or_default(),
        )
    }
}

impl TryFrom<UpsertPolicyRequest> for CompletePolicy {
    type Error = Error;

    fn try_from(value: UpsertPolicyRequest) -> Result<Self, Self::Error> {
        CompletePolicy::new(
            "".to_string(),
            PolicyVersion::try_from(value.version)?,
            PolicyEffect::try_from(&value.effect)?,
            value.actions,
            value.resources.unwrap_or_else(Vec::new),
        )
    }
}

#[post("/policies")]
pub(crate) async fn upsert_policy(
    info: web::Json<UpsertPolicyRequest>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    info.validate()?;
    let policy = CompletePolicy::try_from(info.0)?;

    storage.save_policy(&policy).await?;
    Ok(HttpResponse::Ok().json(policy.to_json()))
}

#[get("/policy/{id}")]
pub(crate) async fn get_policy(
    web::Path(id): web::Path<String>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    let result = storage.find_policy(id).await?;
    match result {
        Option::None => Err(ZephirError::NotFound),
        Option::Some(policy) => Ok(HttpResponse::Ok().json(policy.to_json())),
    }
}
