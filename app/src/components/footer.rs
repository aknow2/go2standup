use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

use crate::ctx::meeting::MeetingContext;

#[function_component(Header)]
pub fn members_list() -> Html {
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
        <nav class="hero is-small is-success">
            <div class="hero-body">
                <div class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <p class="title">
                                {"Standup board"}
                            </p>
                        </div>
                    </div>
                    <div class="level-right">
                        <div class="level-item">
                            <div class="field has-addons">
                                <div class="control">
                                    <input class="input" type="text" value={url} readonly={true} />
                                </div>
                                <div class="control">
                                    <a class="button is-info" onclick={copy}>
                                        <span class="icon is-small">
                                            <i class="material-icons" >{"assignment"}</i>
                                        </span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
