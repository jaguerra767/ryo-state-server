use crate::api::{get_job_progress, get_job_setup, get_node_levels, get_node_weights, get_state, get_status, get_sys_clean_maint, update_job_qty, update_job_setup, update_node_levels, update_node_weight, update_state, update_status, update_sys_cleaning_maintenance};
use axum::http::{HeaderValue, Method};
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use state::RyoState;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

pub mod api;
pub mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let ryo_state = Arc::new(Mutex::new(RyoState::default()));
    let app: Router = Router::new()
        .route("/", get(get_state))
        .route("/JobProgress", get(get_job_progress))
        .route("/NodeLevels", get(get_node_levels))
        .route("/NodeWeights", get(get_node_weights))
        .route("/Status", get(get_status))
        .route("/SystemCleaningMaintenance", get(get_sys_clean_maint))
        .route("/jobSetupStep", get(get_job_setup))
        .route("/", post(update_state))
        .route("/JobQty", post(update_job_qty))
        .route("/NodeLevels", post(update_node_levels))
        .route("/NodeWeights", post(update_node_weight))
        .route("/Status", post(update_status))
        .route(
            "/SystemCleaningMaintenance",
            post(update_sys_cleaning_maintenance),
        )
        .route("/jobSetupStep", post(update_job_setup))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
        .with_state(Arc::clone(&ryo_state));

    let second_app = Router::new().route("/", get(handler));

    let l2 = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let (stx, srx) = tokio::sync::oneshot::channel::<()>();

    let handle = tokio::spawn(async move {
        axum::serve(l2, second_app)
            .with_graceful_shutdown(async move {
                let _ = srx.await;
            })
            .await
            .unwrap();
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    _ = stx.send(());
    _ = handle.await;
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
