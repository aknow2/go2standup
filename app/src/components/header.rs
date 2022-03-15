use yew::prelude::*;

#[function_component(Header)]
pub fn members_list() -> Html {
    html! {
      <nav class="hero is-small is-success">
          <div class="hero-body">
              <p class="title">
                  {"Standup board"}
              </p>
          </div>
      </nav>
    }
}

