use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <div>
            <h1 class="text-xl font-bold tracking-tight text-slate-200 sm:text-2xl">{"Experience"}</h1>
            <div class="container grid grid-cols-[2fr_3fr]">
                <div>
                    <p class="text-gray-500">{"Jul 2023 - present"}</p>
                </div>
                <div class="">
                    <h2 class="font-bold">{"Software Engineer Intern - FOSSEC"}</h2>
                    <p class="text-justify">{"Part of a team of software developers, where I built MVP products and adopted different technologies to meet company objectives."}</p>
                    <p class="text-gray-400">{"Rust • Webassembly • C# • TypeScript • CI/CD"}</p>
                </div>
            </div>
            <div class="container grid grid-cols-[2fr_3fr]">
                <div>
                    <p class="text-gray-500">{"Nov 2022 - Mar 2023"}</p>
                </div>
                <div class="">
                    <h2 class="font-bold">{"Summer Research Intern - IPAS"}</h2>
                    <p class="text-gray-400">{"Institute of Photonics and Advanced Sensing"}</p>
                    <p class="text-justify">{"Joined the Centre of Light for Life to optimise and streamline research processes by customising software to match requirements."}</p>
                    <p class="text-gray-400">{"Rust • Webassembly • C# • TypeScript • CI/CD"}</p>
                </div>
            </div>
        </div>
    }
}