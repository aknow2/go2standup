use stylist::style;
use yew::prelude::*;
use wasm_bindgen::*;
use web_sys::{HtmlInputElement};
use crate::ctx::meeting::{MeetingContext, MeetingActions};

#[function_component(ParkingLot)]
pub fn members_list() -> Html {
  let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");

  let state = meeting_ctx.state.clone();
  let memo = state.memo.to_string();
  let change_memo = {
    let ctx = meeting_ctx.clone();
    Callback::from(move |e: Event| {
      let target = e.target().expect("Event should have a target when dispatched");
      let val = target.unchecked_into::<HtmlInputElement>().value();
      ctx.dispatch(MeetingActions::UpdateMemo(val));
    })
  };
  let textarea = use_state(|| {
        let style = style!(
            r#"
                width: 100%;
                height: 100%;
                padding: 4px;
                border: 3px solid #03A688;
                background-color: #1D3249;
                border: 1px solid #aaa;
            "#
        ).expect("Failed to create style");
        style.get_class_name().to_string()
    });

  html! {
    <textarea
      class={textarea.to_string()}
      placeholder="Rarking lot"
      rows="10"
      value={memo.to_string()}
      onchange={change_memo}
    ></textarea>
  }
}
