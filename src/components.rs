use yew::prelude::*;

const INFO: [(&str, &str); 4] = [
    ("Información", "sobre-mi"),
    ("Experiencia", "experiencia"),
    ("Educación", "educacion"),
    ("Skills", "skills"),
];
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

struct EducationItem {
    title: &'static str,
    institution: &'static str,
    location: &'static str,
    period: &'static str,
    notes: &'static [&'static str],
}

struct SkillCategory {
    name: &'static str,
    items: &'static [&'static str],
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

const EDUCATION: [EducationItem; 2] = [
    EducationItem {
        title: "Técnico en Electrónica",
        institution: "Escuela Técnica N.º 1 de Vicente López",
        location: "Vicente López, Buenos Aires",
        period: "2012 – 2019",
        notes: &[
            "Subcampeón Olimpiada Nacional de Electrónica (2019).",
            "Enfoque en robótica y programación.",
        ],
    },
    EducationItem {
        title: "Ingeniería en Sistemas (en curso)",
        institution: "Universidad Tecnológica Nacional (UTN)",
        location: "Buenos Aires, Argentina",
        period: "2021 – Presente",
        notes: &[],
    },
];

const SKILL_GROUPS: [SkillCategory; 6] = [
    SkillCategory {
        name: "Programación",
        items: &["C/C++", "Rust", "Zig", "Python"],
    },
    SkillCategory {
        name: "Herramientas y flujo de trabajo",
        items: &[
            "FreeRTOS",
            "Sistemas embebidos",
            "Latex",
            "GDB",
            "Makefiles / CMake",
            "Git / GitHub",
        ],
    },
    SkillCategory {
        name: "Diseño y colaboración",
        items: &[
            "KiCad 7.0/8.0",
            "Eagle",
            "EasyEDA",
            "OnShape",
            "Nextcloud",
            "Jira",
            "OpenProject",
            "Suite Ofimática / LibreOffice",
        ],
    },
    SkillCategory {
        name: "Sistemas y redes",
        items: &["Administración Linux", "Sistemas Cisco (Networking)"],
    },
    SkillCategory {
        name: "Electrónica",
        items: &[
            "Diseño de circuitos electrónicos",
            "Uso de osciloscopio",
            "Microsoldadura",
        ],
    },
    SkillCategory {
        name: "Otras",
        items: &["Desarrollo IoT", "Documentación técnica"],
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
                    { for INFO.iter().map(|(label, target)| html! {
                        <tr>
                            <td>
                                <a class="profile_link" href={ format!("#{target}") }>{ *label }</a>
                            </td>
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
        <div id="sobre-mi" class="card">
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
        <section id="experiencia" class="experience_list">
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

#[function_component(EducationSection)]
pub fn education_section() -> Html {
    html! {
        <section id="educacion" class="experience_list">
            { for EDUCATION.iter().map(|edu| html! {
                <article class="card experience_block">
                    <div class="experience_header">
                        <h3>{ edu.title }</h3>
                        <table class="experience_meta_primary">
                            <tbody>
                                <tr>
                                    <th>{"Instituto"}</th>
                                    <td>{ edu.institution }</td>
                                </tr>
                                <tr>
                                    <th>{"Ubicación"}</th>
                                    <td>{ edu.location }</td>
                                </tr>
                                <tr>
                                    <th>{"Período"}</th>
                                    <td>{ edu.period }</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    {
                        if !edu.notes.is_empty() {
                            html! {
                                <ul class="experience_points">
                                    { for edu.notes.iter().map(|note| html! { <li>{ *note }</li> }) }
                                </ul>
                            }
                        } else {
                            Html::default()
                        }
                    }
                </article>
            }) }
        </section>
    }
}

#[function_component(SkillsSection)]
pub fn skills_section() -> Html {
    html! {
        <section id="skills" class="card">
            <h3>{"Skills"}</h3>
            <div class="skills_grid">
                { for SKILL_GROUPS.iter().map(|group| html! {
                    <article class="skill_group">
                        <h4>{ group.name }</h4>
                        <ul>
                            { for group.items.iter().map(|item| html! { <li>{ *item }</li> }) }
                        </ul>
                    </article>
                }) }
            </div>
        </section>
    }
}
