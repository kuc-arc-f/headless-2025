# headless-2025

 Version: 0.9.1

 date    : 2025/08/28 

 update :

***

Rust Headless CMS , workers-rs

* cloudflare Workers-rs , D1 database
* rustc 1.88.0
* cargo 1.88.0

***
### API document

https://github.com/kuc-arc-f/headless-2025/blob/main/headless-2025/document/api.md

***
### setup
* wrangler.toml
* API_KEY: API auth key
* USER_NAME , PASSWORD : login name, password

```
name = "headless-2025"
main = "build/worker/shim.mjs"
compatibility_date = "2023-03-22"

assets = { directory = "./public/" }

[vars]
USER_NAME = "user1@example.com"
PASSWORD = "1234"
API_KEY = "123"

[build]
command = "cargo install -q worker-build && worker-build --release"

[[d1_databases]]
binding = "DB"
database_name = ""
database_id = ""

```
***
* TABLE: table.sql
***
* dev-start
```
npm run build
npm run dev
```
***

