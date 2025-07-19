set shell := ["bash", "-cu"]

# Get the project name from the current directory
project_name := `basename "$PWD"`

default:
  @just --summary

build:
  cargo build --all

test:
  cargo test --all

lint:
  cargo clippy --all-targets -- -D warnings -W clippy::all -W clippy::pedantic -W clippy::nursery -A clippy::module_name_repetitions

fmt-check:
  cargo fmt -- --check

fmt:
  cargo fmt

check:
  just fmt-check
  just lint
  just test

run *args:
  cargo run --release -- 

docker-build:
  docker build -t project_name .

docker-run:
  docker run --rm -it project_name 