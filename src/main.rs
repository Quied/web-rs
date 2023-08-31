#[macro_use] extern crate rocket;

use yew::prelude::*;

mod routes;

// mod pages;

// #[path = "pages/registration.rs"] mod register;
// #[path = "pages/authorization.rs"] mod authorization;

#[launch]
fn rocket() -> _ {


   rocket::build()
     .mount("/", routes![routes::index])

}

