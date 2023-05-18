use axum::{http::StatusCode, routing::get, Router};

/// The main function is async: this is not possible.
/// - async functions are syntactic sugar for functions that return a future
/// - the main function cannot return a future (therefore it cannot be async)
/// - #[tokio::main] is a macro: it creates an actual main function, that calls (and polls) the
///   async main function
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/nono", get(forbidden));

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
