use yew::prelude::*;
use crate::components::aboutme::AboutMe;
#[function_component(Home)]
pub fn home() -> Html{
    html! {
        <AboutMe />
    }
}