# Rocket Rust

### Requirement

Install [https://rustup.rs/](https://rustup.rs/) \
VS Code extentions: Rust-analyzer

### Create new project

`cargo init rocket-app —bin`

> rocket-app is name of your package
> 

> to switch rust version
`rustup default 1.67.0` or `rustup default stable`
> 

file Cargo.toml

`[package]
name = "rocket-app"
version = "0.1.0"
edition = "2021"`

`[dependencies]
rocket = "0.5.0-rc"`

> -rc mean release-candidate
> 

to serialize and deserialize rust data structure we need package `serde_json` and rocket officely have json feature to enable that we must configure in our Cargo.toml like this:
`rocket = { version = "0.5.0-rc", features = ["json"] }`
`serde_json = “1.0”`

### Database ORM

using diesel_cli and sqlite database
`sudo api-get install libsqlite3-dev`

`cargo install diesel_cli --no-default-features --features sqlite`

> for help command just type  `diesel`
> 

setup database sqlite

`diesel setup —database-url ./database.sqlite`

> `diesel setup —database-url <path-file>`
> 

unfortunately in my ubuntu error when install diesel_cli with feature sqlite, then I decide to use diesel_cli postgres

`cargo install diesel_cli --no-default-features --features postgres`

`echo DATABASE_URL=postgres://postgres:admin123@localhost/rocket-app > .env`

`diesel setup`

`diesel migration generate create_rustaceans` → it will creating migration folder that cointains up.sql and down.sql up.sql it’s use for each migration, and down.sql it’s use for revert

to running migration `diesel migration run`

to revert migration `diesel migration redo`

to see migration list `diesel migration list`

to see specific migration list on one database `diesel migration list --database-url=postgres://postgres:admin123@localhost/rocket-app`

to migrate specific database `diesel migration run --database-url=postgres://postgres:admin123@localhost/rocket-app`

to revert specific database migration `diesel migration revert --database-url=postgres://postgres:admin123@localhost/rocket-app`