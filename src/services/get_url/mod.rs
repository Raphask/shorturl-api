use chrono::{ Utc };
use diesel::{ExpressionMethods, RunQueryDsl};
use actix_web::{HttpResponse, Responder, get, web};
use redis::AsyncCommands;
use diesel::prelude::*;

// Importe o DbPool aqui também
use crate::services::{connection::DbPool, schema::url_insert::{self}};

#[get("/{id}")]
pub async fn redirect_to_original(
    path: web::Path<String>,
    pool: web::Data<DbPool>, // 1. Recebe o pool do Actix
) -> impl Responder {
    let url_id = path.into_inner();
    
    // 2. Extrai a conexão
    let mut conn = pool.get().expect("Não foi possível pegar uma conexão do pool");
let client = redis::Client::open("redis://redis:6379").expect("Não foi possível abrir o cliente Redis");
let mut con = client.get_multiplexed_async_connection().await.expect("Não foi possível conectar ao Redis");
    let cached: Option<String> = con.get(&url_id).await.unwrap_or(None);

    match cached {
        Some(url) => {
            HttpResponse::MovedPermanently().append_header(("Location", url)).finish()
        }
        None => {
            // 3. O Diesel funciona normalmente sem mudar nenhuma linha da query
          let url_original = match url_insert::table
    .filter(url_insert::urlshort.eq(&url_id))
    .select((url_insert::urloriginal, url_insert::urlshort, url_insert::expires_at))
    .first::<(String, String, chrono::NaiveDateTime)>(&mut conn)
{
    Ok(url) => url,
    Err(_) => {
        return HttpResponse::NotFound()
            .body("Essa URL não existe no banco. Adicione uma nova.");
    }
};

            if url_original.2 < Utc::now().naive_utc() {
                return HttpResponse::Gone().finish();
            }

            let ttl = (url_original.2 - Utc::now().naive_utc()).num_seconds() as u64;
            let _: () = con.set_ex(&url_id, &url_original.0, ttl).await.expect("msg");

            HttpResponse::MovedPermanently().append_header(("Location", url_original.0)).finish()
        }
    }
}