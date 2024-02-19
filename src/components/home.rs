use crate::components::about::About;
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <About />
    }
}
