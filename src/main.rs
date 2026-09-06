use axum::Router;
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod models;
mod routes;
mod utils;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    postgres_user: String,
    postgres_password: String,
    postgres_db: String,
    app_port: u32,
    postgres_host: String,
    postgres_db_port: u32,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(match EnvFilter::try_from_default_env() {
            Ok(filter) => filter,
            Err(_) => EnvFilter::new("info"),
        })
        .init();

    match dotenv() {
        Ok(_) => tracing::info!("Loading env..."),
        Err(err) => {
            tracing::error!("Failed to load env variables... {:#}", err);
            return;
        }
    }

    // Parse env variables
    let config = match envy::from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("Could not parse config: \n{:?}", e);
            std::process::exit(1);
        }
    };

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres_user,
        config.postgres_password,
        config.postgres_host,
        config.postgres_db_port,
        config.postgres_db
    );

    let pool = match PgPool::connect(&db_url).await {
        Ok(connection_pool) => {
            tracing::info!("Connected to database");
            connection_pool
        }
        Err(_) => {
            tracing::error!("Could not connect to database!");
            std::process::exit(1)
        }
    };

    let state = app::AppState {
        db: app::PostgresDbPool::new(pool),
    };

    let app = Router::new()
        .nest("/api/v1/", routes::app_router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", config.app_port);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(tcp) => tcp,
        Err(_) => return,
    };

    tracing::info!("App is running on http://{:}...", addr);

    match axum::serve(listener, app).await {
        Ok(arg) => tracing::info!("App is shutting down...\n{:?}", arg),
        Err(e) => panic!("{}", e),
    }
}
