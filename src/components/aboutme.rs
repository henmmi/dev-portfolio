use yew::prelude::*;

#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    html! {
        <div>
            <h1 class="text-xl font-bold tracking-tight text-slate-200 sm:text-2xl">
                <a>{"About"}</a>
            </h1>
            <p>{"I am a software developer with a passion for learning and experimenting."}</p>
        </div>
    }
}
