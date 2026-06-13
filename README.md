# shorturl-api

A URL shortener REST API built with Rust, Actix-web, PostgreSQL, Redis, and Docker.

## Tech Stack

- **Rust** — Actix-web 4, r2d2 connection pool
- **PostgreSQL** — persistent URL storage via Diesel ORM
- **Redis** — cache-aside for fast redirects
- **Docker** — fully containerized with Docker Compose

## Features

- Shorten any URL with a randomly generated 6-character code (nanoid)
- Redirect to the original URL via short code
- Configurable expiration per URL (in hours)
- Returns `410 Gone` for expired URLs
- Returns `404 Not Found` for unknown short codes
- Redis cache-aside: redirects are served from cache after the first access
- Rate limiting per IP (in-memory)
- JSON error handling with descriptive messages

## Project Structure

```
src/
├── main.rs
└── services/
    ├── mod.rs
    ├── connection/     # r2d2 PostgreSQL pool
    ├── insert_url/     # POST /url handler
    ├── get_url/        # GET /{id} handler
    ├── models.rs       # Diesel structs
    └── schema.rs       # Diesel schema
```

## Getting Started

### Prerequisites

- Docker
- Docker Compose

### Run

```bash
git clone https://github.com/raphask/shorturl-api
cd shorturl-api
docker compose up -d --build
```

The API will be available at `http://localhost:8081`.

> On first run, the database and table are created automatically via migrations.

## API

### Shorten a URL

```
POST /url
Content-Type: application/json
```

**Body:**
```json
{
  "url": "https://example.com/some/long/url",
  "horas": 24
}
```

**Response:**
```
localhost:8081/abc123
```

### Redirect

```
GET /{short_code}
```

- `301 Moved Permanently` — redirects to the original URL
- `410 Gone` — URL has expired
- `404 Not Found` — short code does not exist

## Environment Variables

Configure in `docker-compose.yml`:

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string |
| `REDIS_URL` | Redis connection string |

## How Caching Works

On the first access to a short URL, the original URL is fetched from PostgreSQL and stored in Redis with a TTL equal to the remaining expiration time. Subsequent requests are served directly from Redis, avoiding database queries.

## License

MIT

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
