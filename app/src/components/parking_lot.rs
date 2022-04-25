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

  html! {
    <div>
      <div>
        <textarea
            class="textarea"
            placeholder="Rarking lot"
            rows="10"
            value={memo.to_string()}
            onchange={change_memo}
        ></textarea>
      </div>
    </div>
  }
}

