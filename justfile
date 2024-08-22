default: 
    @just --list

alias b := build

# could update the protos' crate
build:
    cargo build

run-tag:
    cargo run --package tag-srv

run-post:
    cargo run --package post-srv

run-comment:
    cargo run --package comment-srv

run-admin:
    cargo run --package admin-srv

run-blog:
    cargo run --package blog-api

run-all:
    just run-tag &
    just run-post &
    just run-comment &
    just run-admin &
    just run-blog &
    wait

gen-entity:
    sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/lily-blog -o entity/src/entity