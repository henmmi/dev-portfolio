use crate::components::{home::Home, contact::Contact, projects::Projects, nav::Nav, experience::Experience};
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <header class="py-4"></header>
        <div class="mx-auto min-h-screen max-w-screen-xl px-6 py-12 font-sans md:px-12 md:py-20 lg:px-24 lg:py-0">
            <div class="lg:flex lg:justify-between lg:gap-4">
                <div class="sm:w-1/3  md:1/4 w-full flex-shrink flex-grow-0 p-4">
                    <div class="sticky top-0 p-4 w-full">
                        <ul class="flex sm:flex-col overflow-hidden content-center justify-between">
                            <Nav/>
                        </ul>
                    </div>
                </div>
                <main role="main" class=" sm:w-1/2 w-full h-full flex-grow p-3 overflow-auto flex-col items-center justify-center max-w-2xl">
                    <Home/>
                    <Experience/>
                    <Projects/>
                    <Contact/>
                </main>
            </div>
        </div>
            <footer class="mt-auto">
            </footer>
        </BrowserRouter>
    }
}