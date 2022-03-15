use yew::prelude::*;
use wasm_bindgen::*;
use web_sys::{HtmlInputElement};
use crate::data::member::{ Members };

#[derive(Properties, PartialEq)]
pub struct MeetingProps {
    pub members: Members,
    pub on_back: Callback<()>,
    pub memo: String,
    pub on_change_memo: Callback<String>
}

#[function_component(Meeting)]
pub fn members_list(MeetingProps { members, on_back, memo, on_change_memo }: &MeetingProps) -> Html {
  let back = {
    let on_back = on_back.clone();
    Callback::from(move |_| {
      on_back.emit(());
    })
  };
  let change_memo = {
    let change_memo = on_change_memo.clone();
    Callback::from(move |e: Event| {
        let target = e.target().expect("Event should have a target when dispatched");
        let val = target.unchecked_into::<HtmlInputElement>().value();
        change_memo.emit(val);
    })
  };

  html! {
    <div class="container is-max-widescreen py-3">
        <div class="columns">
            <div class="column">
                <div class="button is-white" onclick={back}>
                    <span class="icon is-large">
                        <i class="material-icons">{"undo"}</i>
                    </span>
                </div>
            </div>
        </div>
        <div class="columns">
            <div class="column">
                {
                    members.iter().enumerate().map(|(i, member)| {
                        let color_class = if i == 0 { "card mt-6 px-3 has-background-warning" } else { "card mt-6 px-3" };
                        html! {
                            <div class={color_class}>
                                <div class="columns is-vcentered">
                                    <div class="column is-size-3">
                                        {member.name.to_string()}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="column">
                <div class="is-size-4">{ "Parking lot" }</div>
                <textarea
                    class="textarea" rows="10"
                    value={memo.to_string()}
                    onchange={change_memo}
                ></textarea>
            </div>
        </div>
    </div>
  }
}

