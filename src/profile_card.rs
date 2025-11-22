use yew::prelude::*;

const INFO: [&str; 4] = ["Información", "Experiencia", "Educación", "Skills"];

#[function_component(ProfileCard)]
pub fn profile_card() -> Html {
    html! {
        <div class="profile_card">
            <img class="profile_avatar" src="https://placehold.co/120x120" alt="avatar" />
            <table class="information_table">
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
