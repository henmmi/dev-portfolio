use crate::components::nav::Nav;
use crate::components::{experience::Experience, home::Home, projects::Projects};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <div class="mx-auto min-h-screen max-w-screen-xl px-6 py-12 md:px-12 md:py-20 lg:px-24 lg:py-0 font-sans">
            <div class="lg:flex lg:gap-4 lg:justify-between">
                <header class="sticky lg:flex-col lg:1/2 w-full lg:top-0 lg:py-24 lg:max-h-screen">
                    <ul class="content-center justify-center items-center">
                        <Nav/>
                    </ul>
                </header>
                <main role="main" class="w-full lg:py-24">
                    <Home/>
                    <Experience/>
                    <Projects/>
                </main>
            </div>
        </div>
        <footer class="mt-auto"></footer>
        </BrowserRouter>
    }
}
