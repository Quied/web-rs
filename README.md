## How it's works
A simple web-app implemented in Rust that include Rocket framework for back-end, and Yew as front-end, open https://127.0.0.1:8000 to view it in browser.

- [Install Rust](https://www.rust-lang.org/tools/install)
- [Rocket](https://rocket.rs/) - Rust framework for backend
- [Yew](https://yew.rs/) - Framework for creating reliable and efficient web applications
- [Diesel](https://diesel.rs) - iteract / manage with databases

## Dependencies
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- `cargo install wasm-pack`
- `cargo install trunk`
- `rustup target add wasm32-unknown-unknown`

## Usage 
- `mkdir my_web && cd my_web`
- `git clone https://github.com/Quied/web_rs.git`
- `cd web_rs`
- `rustup update`
- `rustup default nightly`
- `cargo run`

## Docker
- `sudo systemctl start docker.service`
- `$web_rs docker build -t my-web .`
- `docker run my-web`

## Database

## Structure
 ### src/
 #### |- main.rs // For server start
 #### |- routes.rs // include all pages

### src/pages/
#### |- registretion.rs // registration page
#### |- autorization.rs // autorization page
#### |- users.rs // about users page








