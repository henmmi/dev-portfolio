use implicit_clone::sync::IString;
use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsProps {
    image: IString,
    title: IString,
    description: IString,
    tech: IString,
    url: IString,
}

#[function_component(ProjectItem)]
fn project_item(props: &ProjectsProps) -> Html {
    let url = props.url.to_string();
    html! {
        <div class="rounded-lg hover:bg-gray-800 h-full">
            <a href={url} target="_blank" class="flex flex-col items-center rounded-lg shadow md:flex-row md:max-w-xl h-full">
                <img class="object-cover h-64 w-64 rounded-t-lg md:rounded-none md:rounded-s-lg" src={props.image.to_string()} alt="" />
                <div class="flex flex-col justify-between p-4 leading-normal h-full">
                    <h5 class="mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white">{props.title.to_string()}</h5>
                    <p class="text-sm mb-3 font-normal text-gray-700 dark:text-gray-400">{props.description.to_string()}</p>
                    <p class="text-sm font-normal text-gray-700 dark:text-gray-400">{props.tech.to_string()}</p>
                    <p class="text-sm font-normal text-gray-700 dark:text-gray-400">{"Click to view source code."}</p>
                </div>
            </a>
        </div>
    }
}


#[function_component(Projects)]
pub fn projects() -> Html{
    let valorant_web_radar = ProjectsProps {
        image: IString::from("assets/val_radar.png"),
        title: IString::from("Valorant Web Radar"),
        description: IString::from("A web application that displays agent data and other game information on a radar map."),
        tech: IString::from("Rust, WebAssembly, HTML, CSS (Bootstrap)"),
        url: IString::from("https://github.com/henmmi/valorant-web-radar"),
    };
    let lightsheet_project = ProjectsProps {
        image: IString::from("assets/matlab_logo.png"),
        title: IString::from("Lightsheet GUI"),
        description: IString::from("A desktop application that captures multiple images as a specimen is moved through the focal plane."),
        tech: IString::from("Matlab"),
        url: IString::from("https://github.com/henmmi/lightsheet-gui-matlab"),
    };
    html! {
        <div>
            <h1 class="text-xl font-bold tracking-tight text-slate-200 sm:text-2xl">
                <a>{"Projects"}</a>
            </h1>
            <div class="py-4 space-y-4">
            <ProjectItem image={valorant_web_radar.image.clone()} title={valorant_web_radar.title.clone()} description={valorant_web_radar.description.clone()} tech={valorant_web_radar.tech.clone()} url={valorant_web_radar.url.clone()} />
            <ProjectItem image={lightsheet_project.image.clone()} title={lightsheet_project.title.clone()} description={lightsheet_project.description.clone()} tech={lightsheet_project.tech.clone()} url={lightsheet_project.url.clone()} />
            </div>
        </div>
    }
}