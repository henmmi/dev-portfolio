use implicit_clone::sync::IString;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ExperienceProps {
    #[prop_or_default]
    title: IString,
    #[prop_or_default]
    company: IString,
    #[prop_or_default]
    date: IString,
    #[prop_or_default]
    description: IString,
    #[prop_or_default]
    tech: IString,
}
#[function_component(ExperienceItem)]
pub fn experience_item(
    ExperienceProps {
        title,
        company,
        date,
        description,
        tech,
    }: &ExperienceProps,
) -> Html {
    let company = company.as_str().to_string();
    let title = title.as_str().to_string();
    let date = date.as_str().to_string();
    let description = description.as_str().to_string();
    let tech = tech.as_str().to_string();
    html! {
        <div class="container grid lg:grid-cols-[2fr_3fr] sm:space-py-2 lg:py-4">
            <div>
                <p class="text-gray-500">{date}</p>
            </div>
            <div>
                <h2 class="font-bold">{title}</h2>
                <h3 class="text-gray-400">{company}</h3>
                <p class="text-justify text-sm">{description}</p>
                <p class="text-gray-400 text-sm">{tech}</p>
            </div>
        </div>
    }
}
#[function_component(Experience)]
pub fn experience() -> Html {
    let fossec = ExperienceProps {
        title: IString::from("Software Engineer Intern"),
        company: IString::from("FOSSEC"),
        date: IString::from("Jul 2023 - present"),
        description: IString::from("Part of a team of software developers, where I built MVP products and adopted different technologies to meet company objectives."),
        tech: IString::from("Rust • Leptos • Yew • egui • WebAssembly • C# • TypeScript • HTML • CSS (Tailwind) • MongoDB • CI/CD"),
    };
    let institute = ExperienceProps {
        title: IString::from("Summer Research Intern"),
        company: IString::from("IPAS (Institute of Photonics and Advanced Sensing)"),
        date: IString::from("Nov 2022 - Mar 2023"),
        description: IString::from("Joined the Centre of Light for Life to optimise and streamline research processes by customising software to match requirements."),
        tech: IString::from("MATLAB • Python"),
    };
    html! {
        <div>
            <h1 class="text-xl font-bold tracking-tight text-slate-200 sm:text-2xl">{"Experience"}</h1>
            <ExperienceItem title={&fossec.title} company={&fossec.company} date={&fossec.date} description={&fossec.description} tech={&fossec.tech} />
            <ExperienceItem title={&institute.title} company={&institute.company} date={&institute.date} description={&institute.description} tech={&institute.tech} />
        </div>
    }
}
