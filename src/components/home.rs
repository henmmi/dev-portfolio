use crate::components::aboutme::AboutMe;
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <AboutMe />
    }
}
