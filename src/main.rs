mod components;

use components::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="shell">
            <div class="row">
                <ProfileCard />
                <div class="col">
                    <AboutSection />
                    <WorkExperience />
                    <EducationSection />
                    <SkillsSection />
                </div>

            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
