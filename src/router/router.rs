use crate::repo::{get_applications, Application};
use crate::service::Service;
use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub async fn route(srv: &Service) {
    // initialize tracing
    tracing_subscriber::fmt::init();
    // add shared state
    let shared_state = Arc::new(PoolState {
        pool: srv.repo.clone(),
    });
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/apps", get(get_apps))
        .with_state(shared_state);
    tracing::info!("app router has been initialized");

    let addr = format!("{}:{}", &srv.conf.server.ip, &srv.conf.server.port);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    String::from("index")
}

async fn health() -> String {
    String::from("healthy")
}

#[derive(Clone)]
struct PoolState {
    pool: Pool<Postgres>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Apps {
    data: Vec<Application>,
}

async fn get_apps(State(state): State<Arc<PoolState>>) -> Json<Apps> {
    let apps = get_applications(&state.pool).await.unwrap();
    Json(Apps { data: apps })

    // let resp = Apps { data: apps };
    // println!("apps: {:?}", apps);
}
