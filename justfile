default: 
    @just --list

alias b := build

# could update the protos' crate
build:
    cargo build

run-tag:
    cargo run --package tag-srv