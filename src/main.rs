mod handler_playground;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use handler_playground::{return_error, return_input, return_optional};
use officetracker::{MacStorage, MacUser};

/// The main function is async: this is not possible.
/// - async functions are syntactic sugar for functions that return a future
/// - the main function cannot return a future (therefore it cannot be async)
/// - #[tokio::main] is a macro: it creates an actual main function, that calls (and polls) the
///   async main function
#[tokio::main]
async fn main() {
    let serverctx = ServerContext {
        storage: MacStorage::new(
            &std::env::var("DATABASE_URL").expect("need to provide database url"),
        )
        .await
        .expect("Couldnt connect to database"),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/nono", get(forbidden))
        .route("/office", post(return_input))
        .route("/office2", post(return_optional))
        .route("/office3", post(return_error))
        .route("/api/users", get(read_users))
        .route("/api/users", post(add_user))
        .with_state(serverctx);

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

// Actual api implementation stuff here:

#[derive(Clone)]
struct ServerContext {
    storage: MacStorage,
}

async fn read_users(State(ctx): State<ServerContext>) -> impl IntoResponse {
    let result = ctx.storage.list_users().await;

    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to list users from db".into(),
        );
    }

    return (
        StatusCode::OK,
        serde_json::to_string(&result.unwrap()).unwrap(),
    );
}

#[derive(Debug, serde::Deserialize)]
pub struct MacUserRequest {
    slack_id: String,
    mac_addr: String,
}

async fn add_user(
    State(serverctx): State<ServerContext>,
    extract::Json(user): extract::Json<MacUserRequest>,
) -> impl IntoResponse {
    let result = serverctx
        .storage
        .add_macuser(&officetracker::MacUser {
            user: user.slack_id,
            mac: user.mac_addr,
        })
        .await;

    if result.is_err() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't insert user into database",
        )
    } else {
        (StatusCode::OK, "inserted")
    }
}
