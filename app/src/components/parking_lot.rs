use yew::prelude::*;
use wasm_bindgen::*;
use web_sys::{HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct MeetingProps {
    pub memo: String,
    pub on_change_memo: Callback<String>
}

#[function_component(ParkingLot)]
pub fn members_list(MeetingProps { memo, on_change_memo }: &MeetingProps) -> Html {
  let change_memo = {
    let change_memo = on_change_memo.clone();
    Callback::from(move |e: Event| {
        let target = e.target().expect("Event should have a target when dispatched");
        let val = target.unchecked_into::<HtmlInputElement>().value();
        change_memo.emit(val);
    })
  };

  html! {
    <div>
      <div class="is-size-3">{ "Parking lot" }</div>
      <div>
        <textarea
            class="textarea"
            rows="10"
            value={memo.to_string()}
            onchange={change_memo}
        ></textarea>
      </div>
    </div>
  }
}

