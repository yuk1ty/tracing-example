use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
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

    // info_span!を使って、スパンを作成している。in_scope関数を使うと、
    // このクロージャー内に含まれる処理はすべて、このスパンに入れられる。
    tracing::info_span!("main").in_scope(|| {
        tracing::info!("Server starts");
        heavy_computation();
        tracing::info!("Shutdown server");
    });
}

fn heavy_computation() {
    tracing::info_span!("heavy_computation").in_scope(|| {
        tracing::info!("Computation starts");
        std::thread::sleep(std::time::Duration::from_secs(5));
        tracing::info!("Computation ends");
    });
}
