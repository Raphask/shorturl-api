use actix_web::{App, HttpResponse, HttpServer, web::{self, JsonConfig}};
use actix_web_ratelimit::{RateLimit, config::RateLimitConfig, store::MemoryStore};
use std::sync::Arc;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness}; // <-- IMPORTANTE


use crate::services::{connection::establish_connection, get_url::redirect_to_original, insert_url::insert_url};

pub mod services;

// Esta macro aponta para a sua pasta de migrations padrão do Diesel
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // 1. Cria o Pool de conexões
    let pool = establish_connection();
    
    // 2. EXTRA: Pega uma conexão do pool temporariamente para rodar as migrations
    let mut conn = pool.get().expect("Não foi possível obter conexão para migrations");
    
    println!("Rodando migrations automáticas...");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Falha ao rodar as migrations do banco de dados");
    println!("Migrations aplicadas com sucesso!");

    // 3. Segue o fluxo normal do Actix
    let db_data = web::Data::new(pool);
    let config = RateLimitConfig::default().max_requests(3).window_secs(10);
    let store = Arc::new(MemoryStore::new());

    println!("Servidor rodando em http://0.0.0.0:8081");

    HttpServer::new(move || {
        App::new()
          .app_data(
        JsonConfig::default().error_handler(|err, _req| {
            let msg = format!("Dado inválido: {}", err);
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().body(msg),
            ).into()
        })
    )
            .app_data(db_data.clone())
            .wrap(RateLimit::new(config.clone(), store.clone()))
            .service(insert_url)
            .service(redirect_to_original)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}