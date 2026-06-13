
use chrono::NaiveDateTime;
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};

use crate::services::schema::url_insert;

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = url_insert)]
pub struct Url{
    id: i32,
    pub urloriginal: String,
    urlshort: String, 
    date: NaiveDateTime,
    expires_at: NaiveDateTime
}

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = url_insert)]
pub struct ReadUrl {
    pub urloriginal: String,
    pub urlshort: String,
}
// Struct para inserir novos dados na tabela users
// Usa #[diesel(table_name = users)] para mapear a tabela
#[derive(Insertable)]
#[diesel(table_name = url_insert)]
pub struct InsertUrl<'a> {
    pub urloriginal: &'a str,
    pub urlshort: &'a str,
    pub date: NaiveDateTime,
    pub expires_at: NaiveDateTime


}