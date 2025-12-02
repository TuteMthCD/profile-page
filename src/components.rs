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
            "Desarrollo con Posix 1003.1 y Software en tiempo real (RTOS)",
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
            <img class="profile_avatar" src="https://media.licdn.com/dms/image/v2/D4E03AQHJPm9riNkqXg/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1711079571206?e=1765411200&v=beta&t=myKQhqhDFg06yaMcSHGRItW-EkqMUoGSOOeRWiEgY50" alt="avatar" />
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
            <div class="social_buttons">
                <a class="social_button" href="https://github.com/TuteMthCD" target="_blank" rel="noreferrer">
                    <svg viewBox="0 0 24 24" aria-hidden="true">
                        <path d="M12 0.5a11.5 11.5 0 0 0-3.64 22.43c0.58 0.11 0.79-0.25 0.79-0.56l-0.02-2c-3.22 0.7-3.9-1.55-3.9-1.55-0.53-1.34-1.3-1.7-1.3-1.7-1.06-0.73 0.08-0.72 0.08-0.72 1.17 0.08 1.78 1.2 1.78 1.2 1.04 1.79 2.74 1.27 3.41 0.97 0.11-0.76 0.41-1.27 0.74-1.56-2.57-0.29-5.27-1.28-5.27-5.7 0-1.26 0.45-2.28 1.19-3.08-0.12-0.3-0.52-1.51 0.11-3.15 0 0 0.97-0.31 3.18 1.18a10.9 10.9 0 0 1 5.8 0c2.2-1.49 3.16-1.18 3.16-1.18 0.64 1.64 0.24 2.85 0.12 3.15 0.74 0.8 1.18 1.82 1.18 3.08 0 4.44-2.71 5.41-5.29 5.69 0.42 0.36 0.8 1.07 0.8 2.15l-0.02 3.18c0 0.31 0.21 0.68 0.8 0.56A11.5 11.5 0 0 0 12 0.5z" />
                    </svg>
                    <span>{"GitHub"}</span>
                </a>
                <a class="social_button" href="https://www.linkedin.com/in/matias-civadda-3319a22bb/" target="_blank" rel="noreferrer">
                    <svg viewBox="0 0 24 24" aria-hidden="true">
                        <path d="M4.98 3.5a2.25 2.25 0 1 1 0 4.5 2.25 2.25 0 0 1 0-4.5zM3 9h3.96v12H3zM9.75 9h3.79v1.64h0.05c0.53-1 1.83-2.05 3.77-2.05 4.03 0 4.77 2.66 4.77 6.12V21H18.2v-5.33c0-1.27-0.02-2.9-1.76-2.9-1.77 0-2.04 1.38-2.04 2.8V21H10V9z" />
                    </svg>
                    <span>{"LinkedIn"}</span>
                </a>
            </div>
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
