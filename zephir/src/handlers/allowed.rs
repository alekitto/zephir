use crate::err::ZephirError;
use actix_web::{post, web, HttpResponse};
use libzephir::identity::role::Role;
use libzephir::policy::allowed_result::AllowedOutcome;
use libzephir::policy::policy::ToJson;
use libzephir::storage::StorageManager;
use log::{debug, log_enabled, trace, Level};
use serde::Deserialize;
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct AllowedInfo {
    subject: String,
    action: String,
    resource: Option<String>,
}

impl TryFrom<&Value> for AllowedInfo {
    type Error = ZephirError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let info = value
            .as_object()
            .ok_or(ZephirError::InvalidRequestError)?
            .clone();
        let subject = info
            .get("subject")
            .ok_or(ZephirError::InvalidRequestError)?
            .as_str()
            .ok_or(ZephirError::InvalidRequestError)?
            .to_string();
        let action = info
            .get("action")
            .ok_or(ZephirError::InvalidRequestError)?
            .as_str()
            .ok_or(ZephirError::InvalidRequestError)?
            .to_string();
        let resource = match info.get("resource") {
            None | Some(Value::Null) => None,
            Some(Value::String(str)) => {
                if str.is_empty() {
                    None
                } else {
                    Some(str.to_string())
                }
            }
            _ => return Err(ZephirError::InvalidRequestError),
        };

        Ok(AllowedInfo {
            subject,
            action,
            resource,
        })
    }
}

#[post("/allowed")]
pub(crate) async fn allowed_action(
    body: web::Json<Value>,
    storage: web::Data<StorageManager>,
) -> Result<HttpResponse, ZephirError> {
    let info = AllowedInfo::try_from(&body.0)?;
    let storage = storage.get_ref();
    let identity = storage.find_identity(&info.subject).await?.ok_or_else(|| {
        trace!(
            r#"Identity "{}" not found. Denying access..."#,
            info.subject.as_str()
        );
        ZephirError::AllowedError
    })?;

    if log_enabled!(Level::Trace) {
        trace!(
            r#"Identity "{}" successfull loaded -> {:#?}"#,
            info.subject.as_str(),
            identity
        );
    }

    let action = Option::Some(&info.action);
    let resource = info.resource.as_ref();

    let mut result = identity.allowed(action, resource, &body.0);
    match result.outcome() {
        AllowedOutcome::Denied => {
            trace!(r#"Identity policies denied access. Returning deny result."#);
            Ok(HttpResponse::Forbidden().json(result.to_value()))
        }
        _ => {
            trace!(
                r#"Identity policies {} access. Now evaluating groups policies..."#,
                match result.outcome() {
                    AllowedOutcome::Allowed => "allowed",
                    AllowedOutcome::Abstain => "conditional allow",
                    _ => "",
                }
            );

            let groups = storage.find_groups_for_identity(&identity, false).await?;
            for g in groups {
                result.merge(g.allowed(action, resource, &body.0));
            }

            let mut builder = if result.outcome() == AllowedOutcome::Denied {
                HttpResponse::Forbidden()
            } else {
                HttpResponse::Ok()
            };
            debug!(
                r#"{} access for action "{}" on resource {}"#,
                match result.outcome() {
                    AllowedOutcome::Allowed => "Allowed",
                    AllowedOutcome::Abstain => "Conditional allowed",
                    AllowedOutcome::Denied => "Denied",
                },
                action.unwrap(),
                resource.unwrap_or(&"NULL".to_string())
            );

            Ok(builder.json(result.to_value()))
        }
    }
}
