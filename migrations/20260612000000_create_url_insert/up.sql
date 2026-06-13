CREATE TABLE url_insert (
    id SERIAL PRIMARY KEY,
    urloriginal VARCHAR NOT NULL,
    urlshort VARCHAR NOT NULL,
    date TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL
);