use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let show_more = use_state(|| false);
    let toggle_bio = {
        let show_more = show_more.clone();
        Callback::from(move |_| show_more.set(!*show_more))
    };

    let skills = ["Rust", "Yew", "WebAssembly", "TypeScript", "UI accesible"];
    let socials = [
        ("GitHub", "https://github.com/tute"),
        ("LinkedIn", "https://www.linkedin.com/in/tute"),
        ("Twitter", "https://twitter.com/tutedev"),
    ];

    html! {
        <main class="shell">
            <section class="card">
                <img class="avatar" src="https://avatars.githubusercontent.com/u/1?v=4" alt="avatar" />
                <div class="heading">
                    <p class="eyebrow">{ "Hola, soy" }</p>
                    <h1>{ "Tu Nombre" }</h1>
                    <p class="subtitle">{ "Desarrollador Rust + WebAssembly" }</p>
                </div>

                <p class="summary">
                    { "Me enfoco en construir front-ends rápidos con Yew y servicios Rust seguros. " }
                    { "Este ejemplo es una tarjeta de presentación viva compilada a WebAssembly." }
                </p>

                <section class="skills">
                    <h2>{ "Puntos fuertes" }</h2>
                    <ul>
                        { for skills.iter().map(|skill| html! { <li>{ *skill }</li> }) }
                    </ul>
                </section>

                <section class="social">
                    <h2>{ "Redes" }</h2>
                    <div class="links">
                        { for socials.iter().map(|(label, url)| html! {
                            <a href={*url} target="_blank" rel="noreferrer">{ *label }</a>
                        }) }
                    </div>
                </section>

                <button class="bio-btn" onclick={toggle_bio.clone()}>
                    { if *show_more { "Ocultar mini bio" } else { "Leer mini bio" } }
                </button>
                {
                    if *show_more {
                        html! {
                            <p class="bio">
                                { "Arranqué escribiendo backends en Rust, pero descubrí que " }
                                { "crear interfaces con Yew me permite compartir más con la comunidad. " }
                                { "Fuera de la compu me gusta la fotografía y el ciclismo urbano." }
                            </p>
                        }
                    } else {
                        Html::default()
                    }
                }
            </section>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
