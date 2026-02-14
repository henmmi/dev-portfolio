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
pub fn experience_item(experience: &ExperienceProps) -> Html {
    let ExperienceProps {
        title,
        company,
        date,
        description,
        tech,
    } = experience;

    html! {
        <div class="container grid lg:grid-cols-[1.5fr_3fr] sm:space-py-2 lg:py-4 mb-10">
            <div>
                <p class="text-gray-500">{date.as_str()}</p>
            </div>
            <div>
                <h2>{title.as_str()}</h2>
                <h3 class="text-gray-400">{company.as_str()}</h3>
                <p class="text-sm">{description.as_str()}</p>
                <p class="text-gray-400 text-sm">{tech.as_str()}</p>
            </div>
        </div>
    }
}
#[function_component(Experience)]
pub fn experience() -> Html {
    let praetorian_aeronautics = ExperienceProps {
        title : "Software Engineer".into(),
        company: "Praetorian Aeronautics".into(),
        date: "Oct 2025 - Present".into(),
        description: "Implemented a Command and Control (C2) system for Counter Uncrewed Aircraft Systems (C-UAS) integrating vision tracking capabilities into a human machine interface".into(),
        tech: "Rust • Bevy(egui) • Tokio".into(),
    };
    let company_369labs = ExperienceProps {
        title: "Sofware Engineer".into(),
        company: "369 Labs".into(),
        date: "Apr 2024 - Oct 2025".into(),
        description: "Developed energy management platform, integrating new solar devices (i.e. smart meters, inverters, batteries) and monitoring via embedded devices".into(),
        tech: "Node.js • JavaScript • PostgresSQL • Jira • BitBucket •Jenkins • AWS".into()
    };
    let fossec = ExperienceProps {
        title: "Software Engineer Intern".into(),
        company: "FOSSEC".into(),
        date: "Jul 2023 - present".into(),
        description: "Part of a team of software developers, where I built MVP products and adopted different technologies to meet company objectives.".into(),
        tech: "Rust • Leptos • Yew • egui • WebAssembly • C# • TypeScript • HTML • CSS (Tailwind) • MongoDB • CI/CD".into(),
    };
    let institute = ExperienceProps {
        title: "Summer Research Intern".into(),
        company: "IPAS (Institute of Photonics and Advanced Sensing)".into(),
        date: "Nov 2022 - Mar 2023".into(),
        description: "Joined the Centre of Light for Life to optimise and streamline research processes by customising software to match requirements.".into(),
        tech: "MATLAB • Python".into(),
    };
    html! {
        <div id="experiences-container">
            <h2 class="content-header">{"Experience"}</h2>
            <ExperienceItem ..praetorian_aeronautics/>
            <ExperienceItem ..company_369labs/>
            <ExperienceItem ..fossec />
            <ExperienceItem ..institute />
        </div>
    }
}
