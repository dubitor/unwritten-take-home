use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use data_columns::{DataColumns, ARRAY_AGG_COLUMNS_QUERY};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use env_logger::Env;
use log::info;
use openssl::ssl::{SslConnector, SslMethod};
use polars::prelude::LazyFrame;
use postgres_openssl::MakeTlsConnector;
use server_config::ServerConfig;
use tokio::task;

use crate::errors::UnwrittenError;

mod data_columns;
mod errors;
mod server_config;

#[derive(Clone)]
struct AppState {
    db_connection_pool: Pool,
}

#[post("/")]
async fn print_db(state: web::Data<AppState>) -> Result<HttpResponse, UnwrittenError> {
    // Query database.
    let client: Client = state.db_connection_pool.get().await?;
    let query = ARRAY_AGG_COLUMNS_QUERY;
    info!("Sending database query: {}", query);
    let rows = client.query(query, &[]).await?;
    let data = rows.first().ok_or(UnwrittenError::DataNotFound)?;

    // Convert data to LazyFrame.
    let columns = DataColumns::try_from(data)?;
    let lf = task::spawn_blocking(move || LazyFrame::try_from(columns)).await??;

    // Print LazyFrame in human readable form.
    dbg!(lf.collect()?);

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Setup database connection pool.
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let cfg = ServerConfig::from_env().unwrap();
    let pool = cfg.pg.create_pool(None, connector).unwrap();

    // Initialise logger.
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start web server.
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db_connection_pool: pool.clone(),
            }))
            .service(print_db)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
