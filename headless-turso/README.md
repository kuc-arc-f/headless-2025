# headless-turso

 Version: 0.9.1

 date    : 2025/09/23 

 update :

***

Rust Axum + Turso SDK  , Headless CMS

* rustc 1.90.0
* cargo 1.90.0

***
### API document

https://github.com/kuc-arc-f/headless-2025/blob/main/headless-turso/document/api.md

***
### setup
* .env
* API_KEY: API auth key

```
API_KEY=123
TURSO_DATABASE_URL=""
TURSO_AUTH_TOKEN=
```

***
* TABLE: scheme.sql

```
CREATE TABLE IF NOT EXISTS todo (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  data TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

***
* start
* open: localhost:3000
```
cargo run  --release
```
***

