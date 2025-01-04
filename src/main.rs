use axum::{
    extract::{Path},
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
    Router,
};
use clap::Parser;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::task;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    error_rate: u8,

    #[arg(short, long, required = true)]
    ports: Vec<u16>,

    #[arg(short, long)]
    data_file: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let error_rate = Arc::new(args.error_rate);

    let items = if let Some(file) = args.data_file {
        let data = tokio::fs::read_to_string(file)
            .await
            .expect("Failed to read data file");
        serde_json::from_str::<Vec<Item>>(&data).expect("Failed to parse JSON data")
    } else {
        vec![
            Item { id: 123, name: "hoge".to_string() },
            Item { id: 345, name: "fuga".to_string() },
        ]
    };

    let mut handles = Vec::new();
    for port in args.ports {
        let error_rate = Arc::clone(&error_rate);
        let items = items.clone();
        let handle = task::spawn(async move {
            let app = Router::new().route("/*path", get(move |path| random_response(path, error_rate, items.clone())));

            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            println!("Server running on http://{}", addr);

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn random_response(Path(path): Path<String>, error_rate: Arc<u8>, items: Vec<Item>) -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    let chance: u8 = rand::random();

    if chance < *error_rate {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": "Internal Server Error",
                "path": path
            })),
        )
    } else {
        let selected_item = items.choose(&mut rng).unwrap();

        (
            StatusCode::OK,
            AxumJson(serde_json::json!({
                "message": "Success",
                "path": path,
                "item": selected_item
            })),
        )
    }
}
