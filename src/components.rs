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
