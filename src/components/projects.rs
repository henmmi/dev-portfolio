use implicit_clone::sync::IString;
use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsProps {
    date: IString,
    title: IString,
    description: IString,
    tech: IString,
    url: IString,
}

#[function_component(ProjectItem)]
fn project_item(props: &ProjectsProps) -> Html {
    let url = props.url.to_string();
    html! {
        <div class="container grid lg:grid-cols-[1.5fr_3fr] sm:space-py-2 lg:py-4 mb-10">
                <div>
                    <p class="text-gray-500">{props.date.to_string()}</p>
                </div>
                <div>
                    <a class="flex hover:underline space-x-1.5" href={url} target="_blank">
                            <h2>{props.title.to_string()}</h2>
                            <img class="h-4" src="assets/arrow-white.svg"/>
                    </a>
                    <p class="text-sm font-normal text-gray-200">{props.description.to_string()}</p>
                    <p class="text-sm font-normal text-gray-700 dark:text-gray-400">{props.tech.to_string()}</p>
                </div>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let valorant_web_radar = ProjectsProps {
        date: IString::from("Sep 2023 - Oct 2023"),
        title: IString::from("Valorant Web Radar"),
        description: IString::from(
            "A web application that displays agent data and other game information on a radar map.",
        ),
        tech: IString::from("Rust • WebAssembly • HTML • CSS (Bootstrap)"),
        url: IString::from("https://github.com/henmmi/valorant-web-radar"),
    };
    let lightsheet_project = ProjectsProps {
        date: IString::from("Dec 2022 - Feb 2023"),
        title: IString::from("Lightsheet GUI"),
        description: IString::from("A desktop application that captures multiple images as a specimen is moved through the focal plane."),
        tech: IString::from("MATLAB"),
        url: IString::from("https://github.com/henmmi/lightsheet-gui-matlab"),
    };
    html! {
        <div id="projects-container">
            <h2 class="content-header">{"Projects"}</h2>
            <div class="py-4 space-y-4">
            <ProjectItem date={valorant_web_radar.date.clone()} title={valorant_web_radar.title.clone()} description={valorant_web_radar.description.clone()} tech={valorant_web_radar.tech.clone()} url={valorant_web_radar.url.clone()} />
            <ProjectItem date={lightsheet_project.date.clone()} title={lightsheet_project.title.clone()} description={lightsheet_project.description.clone()} tech={lightsheet_project.tech.clone()} url={lightsheet_project.url.clone()} />
            </div>
        </div>
    }
}
