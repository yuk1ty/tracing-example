use std::net::Ipv6Addr;

use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // トレーシングログの出力設定を行っている。
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_file(true)
                // `.json`を呼び出すと、一旦JSON形式で出力できるようになる。
                .json(),
        )
        .init();

    let app = Router::new().route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", Ipv6Addr::LOCALHOST, 3000))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// ユーザー作成を想定したモックのエンドポイント。ユーザーの作成に成功すると
/// 201 Createdを返し、バリデーションチェックで失敗した場合は400 Bad Request
/// を返す。トレーシングのサンプルなので、関数内で行われる総裁に逐一ログ出力
/// をさせるようにしている。
// 非同期関数にトレーシングを適用する際は、`tracing::instrument`を付与すること
// が推奨されている。
#[tracing::instrument]
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<Option<User>>) {
    match validate_username(&payload.name) {
        Ok(_) => {
            let user = User { name: payload.name };
            // `response.entity`のようにすると、いわゆるフィールドを設定できる。
            // これらは、分散トレーシングのツールで確認したり検索をかけるさいに便利である。
            // `?user`で、`std::fmt::Debug`の出力結果をログに残せる。
            // 詳しい省略記法は`tracing`のドキュメントを参照のこと。
            tracing::info!(response.body=?user, "successfully created user");
            (StatusCode::CREATED, Json(Some(user)))
        }
        Err(err) => {
            // やはり同様に、`error.kind`と`error.message`というディメンジョンを設定している。
            // `%err`のようにすると、`std::fmt::Display`の出力結果をログに残せる。
            tracing::error!(error.kind="validation", error.message=%err);
            (StatusCode::BAD_REQUEST, Json(None))
        }
    }
}

fn validate_username(name: &str) -> Result<(), String> {
    if name.len() < 3 {
        return Err("ユーザー名が短すぎます。4文字以上に設定してください。".to_string());
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Debug, Serialize)]
struct User {
    name: String,
}
