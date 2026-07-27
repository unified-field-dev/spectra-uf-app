//! Protected `/spectra` host: session auth gate + schema-index happy path.
//!
//! Mirrors what a real host does before mounting [`spectra_app::SpectraRoutes`]:
//! deny anonymous traffic under `/spectra`, then serve the schema catalog shape
//! the UI's schema index builds via `spectra-backend::schema_metadata_list`.
//!
//! ## When to use
//! Smoke the `/spectra` auth + schema-index contract without a full Leptos SSR graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-spectra-uf-app
//! cargo run -p protected-spectra-host
//! ```
//!
//! ## Success
//! Stdout prints `protected_spectra_host: OK — /spectra deny/allow + schema index`.
//!
//! ## Look next
//! Mount `<SpectraRoutes />` in a product host; wire Spectra query backends.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

async fn spectra_schemas(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    // Same catalog helper the Leptos schema-index server fn calls after Higgs auth.
    let schemas = spectra_backend::schema_metadata_list();
    Json(serde_json::json!({
        "path": "/spectra",
        "user": session.user_id,
        "schema_count": schemas.len(),
        "schemas": schemas,
    }))
}

fn app() -> Router {
    Router::new()
        .route("/spectra", get(spectra_schemas))
        .route_layer(from_fn(require_session))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    let denied = status_for("/spectra", None).await;
    assert_eq!(denied, StatusCode::UNAUTHORIZED);

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/spectra")
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.expect("body").to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], "/spectra");
    assert_eq!(body["user"], "demo-ops");
    assert!(body["schema_count"].as_u64().is_some());

    println!("protected_spectra_host: OK — /spectra deny/allow + schema index");
}
