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
        <div>
            <div class="w-full max-w-sm">
                <div class="flex flex-col items-center pb-10 pt-4">
                    <img class="w-24 h-24 mb-3 rounded-full shadow-lg" src="assets/bepo.png" alt="Bonnie image"/>
                    <h5 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">{"Henry Nguyen"}</h5>
                    <span class="text-sm text-gray-500 dark:text-gray-400">{"Software Developer"}</span>
                    <div class="flex mt-4 md:mt-6">
                        <nav class="navigation-bar container-fluid">
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
                    </div>
                </div>
            </div>
        </div>
        
    }
}
