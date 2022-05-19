use stylist::Style;
use yew::prelude::*;
use wasm_bindgen::{*, prelude::Closure};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use crate::ctx::meeting::{MeetingContext, MeetingActions};

fn create_textarea_style(height: &str) -> String {
  let str = format!(r#"
      font-size: 18px;
      resize: none;
      width: 100%;
      min-height: 100%;
      height: {};
      border: none;
      padding: 16px;
      border-radius: 10px 0px 0px 10px;
      background: #20283D;
      overflow-y: hidden;
      box-shadow: -5px -5px 10px rgba(0,0,0,0.25),
                  inset 5px 5px 10px rgba(0,0,0,0.25);
      &:focus {{
        outline: none;
      }}
  "#, height);
  let style = Style::new(str).expect("Failed to create style");
  style.get_class_name().to_string()
}

#[function_component(ParkingLot)]
pub fn members_list() -> Html {
  let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
  let textarea_ref = use_node_ref();

  let state = meeting_ctx.state.clone();
  let memo = state.memo.to_string();
  let textarea = use_state(|| {
    create_textarea_style("100%")
  });

  let change_memo = {
    let ctx = meeting_ctx.clone();
    Callback::from(move |e: Event| {
      let target = e.target().expect("Event should have a target when dispatched");
      let val = target.unchecked_into::<HtmlInputElement>().value();
      ctx.dispatch(MeetingActions::UpdateMemo(val));
    })
  };
   
  {
    let textarea_ref = textarea_ref.clone();
    let textarea = textarea.clone();
    use_effect_with_deps(
      |area_ref| {
        let node = area_ref
          .cast::<HtmlTextAreaElement>()
          .expect("textarea_ref not attached to textarea element");

        let cloned_node = node.clone();
        let listener =
        Closure::<dyn Fn(InputEvent)>::wrap(
            Box::new(move |_: InputEvent| {
              let client_height = cloned_node.client_height();
              let scroll_height = cloned_node.scroll_height();
              if scroll_height > client_height {
                let height = format!("{}px", scroll_height);
                textarea.set(create_textarea_style(&height))
              }
            })
        );
        let _ = node.add_event_listener_with_callback("input",  listener.as_ref().unchecked_ref());
        move || {
          let _ = node.remove_event_listener_with_callback("input", listener.as_ref().unchecked_ref());
        }
      },
      textarea_ref,
    );
  }

  html! {
    <textarea
      ref={textarea_ref}
      class={textarea.to_string()}
      placeholder="Rarking lot"
      value={memo.to_string()}
      onchange={change_memo}
    ></textarea>
  }
}
