mod components;

use components::{AboutSection, ProfileCard};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="shell">
            <div class="row">
                <ProfileCard />

                <div class="col">
                    <AboutSection />
                </div>

            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
