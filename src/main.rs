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
use tokio;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 10)] // エラー率のデフォルト値は10%
    error_rate: u8,

    #[arg(short, long, required = true)] // 必須の引数: ポート番号
    port: u16,

    #[arg(short, long)] // レスポンスデータjsonファイルのパス
    data_file: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    details: HashMap<String, String>,
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
            Item {
                id: 123,
                name: "hoge".to_string(),
                details: {
                    let mut details = HashMap::new();
                    details.insert("description".to_string(), "A sample item".to_string());
                    details.insert("category".to_string(), "A".to_string());
                    details.insert("created_at".to_string(), "2025-01-01T12:00:00Z".to_string());
                    details
                },
            },
            Item {
                id: 345,
                name: "fuga".to_string(),
                details: {
                    let mut details = HashMap::new();
                    details.insert("description".to_string(), "Another sample item".to_string());
                    details.insert("category".to_string(), "B".to_string());
                    details.insert("created_at".to_string(), "2025-01-02T12:00:00Z".to_string());
                    details
                },
            },
        ]
    };

    let app = Router::new().route("/*path", get(move |path| random_response(path, error_rate, items.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn random_response(Path(path): Path<String>, error_rate: Arc<u8>, items: Vec<Item>) -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    let chance: u8 = rand::random::<u8>() % 101;

    if chance < *error_rate {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!( {
                "error": "Internal Server Error",
                "path": path
            })),
        )
    } else {
        let selected_item = items.choose(&mut rng).unwrap();

        (
            StatusCode::OK,
            AxumJson(serde_json::json!( {
                "message": "Success",
                "path": path,
                "item": selected_item
            })),
        )
    }
}
