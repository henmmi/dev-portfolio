use yew::prelude::*;
#[function_component(Contact)]
pub fn contact() -> Html{
    html! {
        <div>
            <h1 class="text-3xl font-bold tracking-tight text-slate-200 sm:text-4xl">{"Contact Me"}</h1>
            <p>{"You can contact me at: "}
                <a href="mailto:henrynguyen.adelaide@outlook.com" class="font-bold">{"henrynguyen.adelaide@outlook.com"}</a>
            </p>
        </div>
    }
}