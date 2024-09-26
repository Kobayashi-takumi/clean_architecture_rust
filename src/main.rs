mod domain;
mod infrastructure;
mod interface_adapter;
mod shared;

#[tokio::main]
async fn main() -> shared::error::Result<()> {
    // .envの読み込み
    dotenv::dotenv().ok();
    // Loggerの初期化
    // shared::logger::setup()?;
    tracing_subscriber::fmt::init();
    // envからConfigを作成
    let config = shared::config::Config::from_env()?;
    // マイグレーションの実行
    // envにマイグレーションのパスが設定されている場合に、マイグレーションが実行される
    if let Some(val) = &config.migrations_path {
        let pool = infrastructure::database::pool(&config).await?;
        infrastructure::database::migration(&pool, val).await?;
        pool.close().await;
    };
    // graphqlサーバ起動
    infrastructure::graphql::server::start(&config).await
}
