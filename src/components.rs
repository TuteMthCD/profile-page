use yew::prelude::*;

const INFO: [&str; 4] = ["Información", "Experiencia", "Educación", "Skills"];
const ABOUT_ROWS: [(&str, &str); 7] = [
    ("Nombre completo", "Matias Nahuel Civadda"),
    ("Rol Actual", "Software Developer (Embedded Software)"),
    ("Estudios", "Técnicatura en Electrónica"),
    ("Carreras en curso", "Ingeniería en Sistemas"),
    ("Ciudad", "Buenos Aires, Argentina"),
    ("Email", "matias.civadda2342001@gmail.com"),
    ("Teléfono", "11-3051-2341"),
];

struct WorkItem {
    role: &'static str,
    company: &'static str,
    period: &'static str,
    location: &'static str,
    points: &'static [&'static str],
}

const WORK_EXPERIENCE: [WorkItem; 5] = [
    WorkItem {
        role: "Desarrollador de Software Embebido",
        company: "STAMM VEGH S.A.U.",
        period: "2024 – Presente",
        location: "Buenos Aires, Argentina",
        points: &[
            "Desarrollo de software embebido en C y C++.",
            "Desarrollo de software sobre Linux utilizando C.",
            "Programación de microcontroladores (ARM, AVR, ESPRESSIF).",
            "Diseño y optimización de firmware para dispositivos IoT.",
            "Integración de protocolos (I2C, SPI, UART, CAN, RS485, RS232).",
            "Pruebas de integración de software embebido.",
        ],
    },
    WorkItem {
        role: "Freelancer",
        company: "Independiente / Remoto",
        period: "2024 – Presente",
        location: "Buenos Aires, Argentina",
        points: &[
            "Desarrollo freelance de software embebido, firmware y soluciones a medida.",
            "Programación en Rust, C++ y stack web (JavaScript/TypeScript, APIs).",
            "Implementación de algoritmos de control y optimización para sistemas embebidos.",
            "Diseño de software e IoT completo, de hardware a backend.",
        ],
    },
    WorkItem {
        role: "Técnico Electrónico",
        company: "STAMM VEGH S.A.U.",
        period: "2023 – 2024",
        location: "Buenos Aires, Argentina",
        points: &[
            "Diseño esquemático, fabricación y testeo de PCBs.",
            "Microsoldadura de placas SMD y THT.",
            "Montaje de tableros eléctricos y sistemas de automatización.",
            "Diseño de sistemas de control para impresoras 3D de resina.",
            "Coordinación de tareas interdisciplinarias (electrónica, microfluídica, biología).",
        ],
    },
    WorkItem {
        role: "Servicio Técnico",
        company: "Keops Cels",
        period: "2019 – 2023",
        location: "Buenos Aires, Argentina",
        points: &[
            "Microsoldadura: BGA, QFN, SOIC, THT, SMD, TO.",
            "Reparación de smartphones, computadoras, audio y placas electrónicas.",
        ],
    },
    WorkItem {
        role: "Pasantía Profesional",
        company: "SIEMENS S.A.",
        period: "2018 – 2019",
        location: "Buenos Aires, Argentina",
        points: &[
            "Práctica de automatización con PLC Siemens básicos.",
            "Armado de aerogeneradores.",
        ],
    },
];

#[function_component(ProfileCard)]
pub fn profile_card() -> Html {
    html! {
        <div class="profile_card">
            <img class="profile_avatar" src="https://placehold.co/120x120" alt="avatar" />
            <table class="information_table">
                <thead>
                    {"Matias Civadda"}
                </thead>
                <tbody>
                    { for INFO.iter().map(|item| html! {
                        <tr>
                            <td>{ *item }</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}

#[function_component(AboutSection)]
pub fn about_section() -> Html {
    html! {
        <div class="card">
            <h3>{"Sobre mi"}</h3>
            <table class="about_table">
                <tbody>
                    { for ABOUT_ROWS.iter().map(|(label, value)| html! {
                        <tr>
                            <th>{ *label }</th>
                            <td>{ *value }</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}

#[function_component(WorkExperience)]
pub fn work_experience() -> Html {
    html! {
        <section class="experience_list">
            { for WORK_EXPERIENCE.iter().map(|item| html! {
                <article class="card experience_block">
                    <div class="experience_header">
                        <h3>{ item.role }</h3>
                        <table class="experience_meta_primary">
                            <tbody>
                                <tr>
                                    <th>{"Empresa"}</th>
                                    <td>{ item.company }</td>
                                </tr>
                                <tr>
                                    <th>{"Periodo"}</th>
                                    <td>{ item.period }</td>
                                </tr>
                                <tr>
                                    <th>{"Ubicación"}</th>
                                    <td>{ item.location }</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <ul class="experience_points">
                        { for item.points.iter().map(|point| html! { <li>{ *point }</li> }) }
                    </ul>
                </article>
            }) }
        </section>
    }
}
