// use rocket::fs::NamedFile;

use yew::prelude::*;
use rocket::response::content;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="d-flex justify-content-center m-5">
                <h1>{"Building a Website in Rust"}</h1>
            </div>
        }
    }
}

#[get("/")]
pub fn index() -> &'static str {

    "Hello World"

    // let page = simple_page();
    // let renderer = page.into_string();

    // Html(renderer)
}