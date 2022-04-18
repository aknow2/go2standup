use yew::prelude::*;

use crate::ctx::meeting::MeetingContext;

#[function_component(Header)]
pub fn members_list() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    let id = meeting_ctx.state.id.clone();
    let origin = web_sys::window().unwrap().location().origin().unwrap();
    let url = origin + "/?id=" + &id;

    let copy = {
        let url = url.clone();
        Callback::from(move |_| {
            let nav = web_sys::window().unwrap().navigator();
            let clip = nav.clipboard().unwrap();
            clip.write_text(&url);
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
