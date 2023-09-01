use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1 style="text-align: center; font-size: 30px;"> 
            {
                "Rustacean" 
            }
            </h1> <hr/>

            <div class="Container">
                <h1 style="text-align: center;  font-size: 20px;">{ "Login" }</h1>
                <input type="text" id="name" name="name" minlength="4" maxlength="10" size="10" />
            </div>

            <div>
                <h1 style="text-align: center; font-size: 20px; ">{ "Password" }</h1>
                <input type="text" id="name" name="name" minlength="4" size="10" />
            </div>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();

}
