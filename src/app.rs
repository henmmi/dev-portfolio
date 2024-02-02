use crate::components::{home::Home, contact::Contact, projects::Projects, nav::Nav};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route{
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
}
/// This is the switch function. It is used by the yew_router::Switch component to determine which component to render based on the current route.
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Projects => html! {<Projects/>},
        Route::Contact => html! {<Contact/>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav/>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}