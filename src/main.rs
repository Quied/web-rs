use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div>
                <h1 id="PreviewText">{ "Login [4 - 10]" }</h1>
                <input type="text" id="name" name="name" minlength="4" maxlength="10" size="10" />
            </div>
            <div>
                <h1>{ "Password" }</h1>
                <input type="text" id="name" name="name" minlength="4" maxlength="8" size="10" />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();

}
