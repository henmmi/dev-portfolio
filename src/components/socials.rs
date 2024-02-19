use yew::prelude::*;

#[function_component(Socials)]
pub fn socials() -> Html {
    html! {
        <div>
            <h2 class="mt-4">{"Links"}</h2>
            <ul class="flex items-center space-x-4">
                <li>
                    <a href="https://www.linkedin.com/in/henrynguyenadl/" target="_blank">
                        <img class="h-10" src="assets/linkedin-icon.svg" alt="LinkedIn"/>
                    </a>
                </li>
                <li>
                    <a href="https://github.com/henmmi" target="_blank">
                        <img class="h-10" src="assets/github-icon.svg" alt="GitHub"/>
                    </a>
                </li>
                <li>
                    <a href="mailto:henrynguyen.adelaide@outlook.com" target="_blank">
                        <img class="h-10" src="assets/mail-icon.svg" alt="Email"/>
                    </a>
                </li>
            </ul>
        </div>
    }
}
