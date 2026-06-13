

use diesel::table;   

table! {
    url_insert (id) {
        id -> Integer,
        urloriginal -> Varchar,
        urlshort -> Varchar,
        date -> Timestamp,
        expires_at -> Timestamp
    
    
    }
}