mod profile_card;

use profile_card::ProfileCard;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="shell">
        <ProfileCard/>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
