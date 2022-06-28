use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

use crate::ctx::{meeting::MeetingContext, styles::StyleContext};

#[function_component(Header)]
pub fn members_list() -> Html {
    let style_ctx = use_context::<StyleContext>().expect("no ctx found");
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    let id = meeting_ctx.state.id.clone();
    let origin = web_sys::window().unwrap().location().origin().unwrap();
    let url = match id {
        Some(id) => origin + "/?id=" + &id,
        None => String::from("-")
    };
    let copy = {
        let url = url.clone();
        Callback::from(move |_| {
            let url = url.clone();
            spawn_local(async move {
                let nav = web_sys::window().unwrap().navigator();
                let clip = nav.clipboard().unwrap();
                JsFuture::from(clip.write_text(&url)).await.unwrap();
            })
        })
    };

    html! {
        <nav class={style_ctx.header.to_string()}>
            <div>
                <p>
                    {"LIVEN"}
                </p>
            </div>
            <div>
                <button
                    class={style_ctx.outline_btn.clone()}
                    onclick={copy}
                >
                    {"Share"}
                </button>
            </div>
        </nav>
    }
}
