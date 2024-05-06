use axum::{extract::State, routing::post, Json, Router};
use config::ENV;
use mongodb::{bson::Document, Client};
use tokio::net::TcpListener;

mod config;

#[derive(Clone)]
struct AppState {
    client: Client,
}
#[tokio::main]
async fn main() {
    let client = Client::with_uri_str(ENV.mongo_url.as_str()).await.unwrap();
    let app = Router::new()
        .route("/log", post(log))
        .with_state(AppState { client });
    let listener = TcpListener::bind(&ENV.logger_uri)
        .await
        .unwrap();

    println!("Listening on http://{}", ENV.logger_uri);
    axum::serve(listener, app).await.unwrap()
}

async fn log(State(state): State<AppState>, Json(payload): Json<Document>) {
    let collection = state.client.database("logs").collection("logs");
    println!("{:?}", payload);
    collection.insert_one(payload, None).await.unwrap();
}
