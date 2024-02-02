use yew::prelude::*;

#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    html! {
        <div>
            <h1 class="text-4xl font-bold tracking-tight text-slate-200 sm:text-5xl">
                <a>{"Henry Nguyen"}</a>
            </h1>
            <p>{"I am a software developer with a passion for learning and teaching."}</p>
        </div>
    }
}
