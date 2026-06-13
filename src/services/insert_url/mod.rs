use chrono::{Utc};
use diesel::{RunQueryDsl};
use actix_web::{HttpResponse, Responder, post, web};
use nanoid::nanoid;
use redis::AsyncCommands;
use serde::Deserialize;
// Importe o tipo DbPool que criamos no arquivo de conexão
use crate::services::{connection::DbPool, models::{InsertUrl}, schema::url_insert::{self}};

#[derive(Deserialize)]
pub struct UrlInput {
    pub url: String,
    pub horas : i64
}

#[post("/url")]
pub async fn insert_url(
    pool: web::Data<DbPool>, // 1. O Actix injeta o pool aqui
    url: web::Json<UrlInput>
) -> impl Responder {
    
    // 2. EXTRAI UMA CONEXÃO DO POOL (É aqui que a mágica acontece!)
    // O seu erro some aqui, porque 'conn' agora é do tipo PgConnection, que o Diesel aceita
    let mut conn = match pool.get() {
    Ok(c) => c,
    Err(_) => return HttpResponse::InternalServerError().body("Erro ao conectar no banco"),
};
  
let client = match redis::Client::open("redis://redis:6379") {
    Ok(c) => c,
    Err(_) => return HttpResponse::InternalServerError().body("Erro ao conectar no Redis"),
};

let mut con = match client.get_multiplexed_async_connection().await {
    Ok(c) => c,
    Err(_) => return HttpResponse::InternalServerError().body("Erro ao obter conexão Redis"),
};

    let urlshort = nanoid!(6);
    let horaconvertida = Utc::now().naive_utc() + chrono::Duration::hours(url.horas);
    
    let new_url = InsertUrl {
        urloriginal: url.url.as_str(),
        urlshort: &urlshort,
        date:  Utc::now().naive_utc(),
        expires_at: horaconvertida
    };
    
    // 3. O resto do código do Diesel continua EXATAMENTE IGUAL
    match diesel::insert_into(url_insert::table)
    .values(&new_url)
    .execute(&mut conn)
{
    Ok(_) => {},
    Err(e) => return HttpResponse::InternalServerError().body(format!("Erro ao salvar: {}", e)),
};

let _: () = match con.set_ex(&urlshort, &url.url, 3600u64).await {
    Ok(v) => v,
    Err(e) => return HttpResponse::InternalServerError().body(format!("Erro no Redis: {}", e)),
};

    HttpResponse::Ok().body(format!("localhost:8081/{}", &urlshort.clone()))
}