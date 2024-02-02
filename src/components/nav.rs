/// This file contains the navigation bar component.
use yew::prelude::*;
use yew_router::prelude::Link;
use crate::app::Route;

/// This struct contains the data for the navigation bar.
struct NavigationBar{
    link: Route,
    string: String,
    is_active: bool,
    id: u32,
}

/// This function contains the navigation bar component.
#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(||{
        vec![
            NavigationBar{
                link: Route::Home,
                string: "Home".to_string(),
                is_active: false,
                id: 0,
            },
            NavigationBar{
                link: Route::Projects,
                string: "Projects".to_string(),
                is_active: false,
                id: 1,
            },
            NavigationBar{
                link: Route::Contact,
                string: "Contact".to_string(),
                is_active: false,
                id: 2,
            },
        ]
    });
    html! {
        <nav class="navigation-bar">
        <Link<Route> classes={classes!("logo")} to={Route::Home}>
        <img src="img/box.png"/>
        </Link<Route>>
        <ul class="nav-list">
        {
            nav_items.iter().map(|nav_item| {
                html!{<li key={nav_item.id} class={classes!("nav-item", if nav_item.is_active { "active" } else { "" })}>
                <Link<Route> to={nav_item.link.clone()}>{nav_item.string.clone()}</Link<Route>>
                    </li>}
            }).collect::<Html>()
        }
        </ul>
        </nav>
    }
}
