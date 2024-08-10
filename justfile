default: 
    @just --list

alias b := build

# could update the protos' crate
build:
    cargo build

run-tag:
    cargo run --package tag-srv

gen-entity:
    sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/lily-blog -o entity/src/entity