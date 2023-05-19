use axum::{
    extract::{self, rejection::JsonRejection},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

/// The main function is async: this is not possible.
/// - async functions are syntactic sugar for functions that return a future
/// - the main function cannot return a future (therefore it cannot be async)
/// - #[tokio::main] is a macro: it creates an actual main function, that calls (and polls) the
///   async main function
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/nono", get(forbidden))
        .route("/office", post(return_input))
        .route("/office2", post(return_optional))
        .route("/office3", post(return_error));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Hello world response for axum (simple string)
/// String is an owned value
/// - Difference between &str and String
async fn index() -> String {
    "hello world".into()
    // or:
    // "hello world".to_owned()
}

async fn forbidden() -> (StatusCode, String) {
    (StatusCode::FORBIDDEN, "forbidden".to_owned())
}

#[derive(Debug, serde::Deserialize)]
struct Device {
    slack_id: String,
    mac_addr: String,
}

#[derive(Debug, serde::Serialize)]
struct OfficeStatus {
    slack_id: String,
    at_office: bool,
}

/// - Explain some pattern matching here
/// - extract::Json is not the same as Json (axum::extract::Json vs axum::Json)
/// - The naming convention for this is not too relevant for this.. this is a library thing
async fn return_input(extract::Json(device): extract::Json<Device>) -> Json<OfficeStatus> {
    let status = OfficeStatus {
        slack_id: device.slack_id,
        at_office: false,
    };

    Json(status)
}

/// Option types
async fn return_optional(payload: Option<Json<Device>>) -> Json<OfficeStatus> {
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
async fn return_error(payload: Result<extract::Json<Device>, JsonRejection>) -> Json<OfficeStatus> {
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
