use yew::prelude::*;
use web_sys::{HtmlInputElement};
use wasm_bindgen::*;
use crate::ctx::meeting::{MeetingActions, MeetingContext};
use crate::data;
use crate::components::member_list::MembersList;

#[function_component(PrepareMembers)]
pub fn prepare_members() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    log::info!("{:?}", meeting_ctx.state);

    let state = meeting_ctx.state.clone();
    let members = state.members.to_vec();
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let leader_id = state.leader_id.clone();

    let add_member = {
        let new_member_name = new_member_name.clone();
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::AddMember(new_member_name.to_string()));
            new_member_name.set(String::from(""))
        })
    };

    let remove_member = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |member: data::meeting::Member| {
            ctx.dispatch(MeetingActions::RemoveMember(member.id.to_string()));
        })
    };

    let shuffle_members = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::ShuffleMembers);
        })
    };

    let new_leader = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::NewLeader);
            log::info!("Start loot ");
        })
    };

    let keydown = {
        let new_member_name = new_member_name.clone();
        let ctx = meeting_ctx.clone();
        Callback::from(move |e: KeyboardEvent| {
            log::info!("on key down {:?}", e.key_code());
            if e.key_code() == 13 {
                ctx.dispatch(MeetingActions::AddMember(new_member_name.to_string()));
                new_member_name.set(String::from(""))
            }
        })
    };

    let change_new_member_name = {
        let new_member_name = new_member_name.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().expect("Event should have a target when dispatched");
            let val = target.unchecked_into::<HtmlInputElement>().value();
            new_member_name.set(val);
        })
    };
    html!{
        <div>
            <div class="block">
                <MembersList leader_id={leader_id.clone()} members={members.to_vec()} on_remove={remove_member} />
            </div>
            <div class="columns is-vcentered">
                <div class="column is-four-fifths">
                    <input
                        class="input is-large"
                        type="text"
                        placeholder="name"
                        value={new_member_name.to_string()}
                        onkeydown={keydown}
                        oninput={change_new_member_name}
                    />
                </div>
                <div class="column is-half">
                    <div class="button is-white" onclick={add_member}>
                        <span class="icon is-large">
                            <i class="material-icons">{"add"}</i>
                        </span>
                    </div>
                </div>
            </div>
            <div class="block buttons">
                <button onclick={new_leader} class="button is-primary is-light">{ "Today's Leader" }</button>
                <button onclick={shuffle_members} class="button is-link is-light">{ "Shuffle order" }</button>
            </div>
        </div>
    }
}
