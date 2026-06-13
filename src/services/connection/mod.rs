use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::{thread, time::Duration};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let mut attempts = 0;
    loop {
        // Criamos o manager dentro do loop para poder movê-lo caso o build falhe
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        
        match r2d2::Pool::builder().build(manager) {
            Ok(pool) => return pool,
            Err(e) => {
                attempts += 1;
                if attempts >= 5 {
                    panic!("Não foi possível criar o pool de banco de dados: {}", e);
                }
                println!("Banco ainda não disponível, tentando novamente em 2 segundos... (Erro: {})", e);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}