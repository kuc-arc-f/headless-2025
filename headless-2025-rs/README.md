# headless-2025-rs

 Version: 0.9.1

 date    : 2025/09/04 

 update :

***

Rust Axum , Headless CMS

* SQLite database
* rustc 1.88.0
* cargo 1.88.0

***
### setup
* .env
* API_KEY: API auth key
* USER_NAME , PASSWORD : login name, password

```
API_KEY=123
USER_NAME = "user1@example.com"
PASSWORD = "1234"
```
***
* TABLE: scheme.sql
* db create
```
sqlite3 cms.db
```
* create table
```
CREATE TABLE IF NOT EXISTS hcm_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content  TEXT NOT NULL,
    data  TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
```
***
* dev-start
* open: localhost:3000
```
npm run build
npm run dev
```
***

