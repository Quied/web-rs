## How it's works
A simple web-app implemented in Rust that include Rocket framework for back-end, and Yew as front-end, open https://127.0.0.1:8000 to view it in browser.

## Dependecies
cargo install wasm-pack
cargo install trunk
rustup target add wasm32-unknown-unknown

## Usage 
- $ mkdir my_web && cd my_web
- $ git clone https://github.com/Quied/web_rs.git
- $ cd web_rs
- $ rustup update
- $ cargo run

## Docker

- $ docker-compose up
- $ docker-compose run web_rs

## Database

## Structure
 src/
 |- main.rs // For server start
 |- routes.rs // include all pages

src/pages/
|- registretion.rs // registration page
|- autorization.rs // autorization page
|- users.rs // about users page








