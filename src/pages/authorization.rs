use yew::prelude::*;

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HomePage
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Hello from Yew!"}</h1>
                <p>{"Communication with Rocket back end:"}</p>
                <button onclick=|_| { }>
                    {"Send Request"}
                </button>
            </div>
        }
    }
}