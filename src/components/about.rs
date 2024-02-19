use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div id="about" class="mt-8 mb-14">
            <h2>{"About"}</h2>
            <p>{"I am a software developer with a passion for learning and experimenting."}</p>
        </div>
    }
}
