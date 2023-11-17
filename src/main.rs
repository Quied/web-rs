#[macro_use] extern crate rocket;

use yew::prelude::*;
use crate::routes::App;
mod routes;
// mod pages;

// #[path = "pages/registration.rs"] mod register;
// #[path = "pages/authorization.rs"] mod authorization;

// #[launch]
fn main() {

  yew::Renderer::<App>::new().render();

  //  rocket::build()
  //    .mount("/", routes![routes::index])

}

