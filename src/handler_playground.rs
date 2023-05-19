use axum::{
    extract::{self, rejection::JsonRejection},
    Json,
};

#[derive(Debug, serde::Deserialize)]
pub struct Device {
    slack_id: String,
    mac_addr: String,
}

#[derive(Debug, serde::Serialize)]
pub struct OfficeStatus {
    slack_id: String,
    at_office: bool,
}

/// - Explain some pattern matching here
/// - extract::Json is not the same as Json (axum::extract::Json vs axum::Json)
/// - The naming convention for this is not too relevant for this.. this is a library thing
pub async fn return_input(extract::Json(device): extract::Json<Device>) -> Json<OfficeStatus> {
    let status = OfficeStatus {
        slack_id: device.slack_id,
        at_office: false,
    };

    Json(status)
}

/// Option types
pub async fn return_optional(payload: Option<Json<Device>>) -> Json<OfficeStatus> {
    let status = if let Some(Json(device)) = payload {
        OfficeStatus {
            slack_id: device.slack_id,
            at_office: false,
        }
    } else {
        OfficeStatus {
            slack_id: "researchableoffice".to_owned(),
            at_office: true,
        }
    };

    Json(status)
}

/// Error pattern matching
pub async fn return_error(
    payload: Result<extract::Json<Device>, JsonRejection>,
) -> Json<OfficeStatus> {
    let status = match payload {
        Ok(Json(device)) => OfficeStatus {
            slack_id: device.slack_id,
            at_office: false,
        },

        Err(JsonRejection::JsonDataError(_)) => {
            eprintln!("missing fields, but i decided to always return an OfficeStatus json so i can't really do anything with this :D");
            OfficeStatus {
                slack_id: "none".to_owned(),
                at_office: false,
            }
        }

        Err(_) => OfficeStatus {
            slack_id: "none".to_owned(),
            at_office: false,
        },
    };

    Json(status)
}
