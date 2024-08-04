default: 
    @just --list

alias b := build;

# could update the protos' crate
build:
    cargo build

