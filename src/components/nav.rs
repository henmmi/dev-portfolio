use crate::components::socials::Socials;
/// This file contains the navigation bar component.
use yew::prelude::*;

/// This function contains the navigation bar component.
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div>
            <div class="sticky top-0 w-full max-w-sm">
                <div class="grid grid-cols-[1fr_3fr] justify-between items-center">
                    <img class="w-20 h-20 rounded-full shadow-lg" src="assets/bepo.png" alt="Profile"/>
                    <div>
                        <h1 class="font-bold text-3xl md:text-xl lg:text-2xl leading-8">{"Henry Nguyen"}</h1>
                        <p class="text-sm text-gray-500 dark:text-gray-400">{"Software Engineer"}</p>
                    </div>
                </div>
                <div class="hidden lg:flex mt-4 lg:mt-6">
                    <nav>
                    <ul class="flex-col">
                        <li>
                            <a class="flex py-2 text-sm font-medium text-gray-900 dark:text-white hover:underline space-x-1.5" href="#about">
                                <h2>{"About"}</h2>
                                <img class="h-5" src="assets/arrow-right.svg"/>
                            </a>
                        </li>
                        <li>
                            <a class="flex py-2 text-sm font-medium text-gray-900 dark:text-white hover:underline space-x-1.5" href="#experiences-container">
                                <h2>{"Experience"}</h2>
                                <img class="h-5" src="assets/arrow-right.svg"/>
                            </a>
                        </li>
                        <li>
                            <a class="flex py-2 text-sm font-medium text-gray-900 dark:text-white hover:underline space-x-1.5" href="#projects-container">
                                <h2>{"Projects"}</h2>
                                <img class="h-5" src="assets/arrow-right.svg"/>
                            </a>
                        </li>
                    </ul>
                    </nav>
                </div>
                <div class="bottom-0">
                    <Socials/>
                </div>
            </div>
        </div>

    }
}
