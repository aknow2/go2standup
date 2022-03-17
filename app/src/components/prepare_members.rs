use yew::prelude::*;
use web_sys::{HtmlInputElement};
use wasm_bindgen::*;
use crate::data;
use crate::components::member_list::MembersList;


#[derive(Properties, PartialEq)]
pub struct PrepareMembersProps {
    pub members: data::member::Members,
    pub leader_id: Option<String>,
    pub new_member_name: String,
    pub on_change_new_member_name: Callback<String>,
    pub on_remove: Callback<data::member::Member>,
    pub on_add_member: Callback<()>,
    pub on_shuffle: Callback<()>,
    pub on_loot_leader: Callback<()>
}

#[function_component(PrepareMembers)]
pub fn prepare_members(PrepareMembersProps {
        leader_id,
        members,
        on_add_member,
        on_remove,
        new_member_name,
        on_change_new_member_name,
        on_shuffle,
        on_loot_leader,
    }: &PrepareMembersProps) -> Html {
    let  add_member = {
        let on_add_member = on_add_member.clone();
        Callback::from(move |_| {
            on_add_member.emit(());
        })
    };
    let keydown = {
        let on_add_member = on_add_member.clone();
        Callback::from(move |e: KeyboardEvent| {
            log::info!("on key down {:?}", e.key_code());
            if e.key_code() == 13 {
                on_add_member.emit(());
            }
        })
    };
    let shuffle = {
        let on_shuffle = on_shuffle.clone();
        Callback::from(move |_| {
            on_shuffle.emit(());
        })
    };
    let loot = {
        let on_loot = on_loot_leader.clone();
        Callback::from(move |_| {
            on_loot.emit(());
        })
    };
    let change_new_member_name = {
        let on_change_new_member_name = on_change_new_member_name.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().expect("Event should have a target when dispatched");
            let val = target.unchecked_into::<HtmlInputElement>().value();
            on_change_new_member_name.emit(val);
        })
    };
    html!{
        <div>
            <span class="is-size-3 block">{ "Reporting order" }</span>

            <div class="block">
                <MembersList leader_id={leader_id.clone()} members={members.to_vec()} on_remove={on_remove} />
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
                <button onclick={loot} class="button is-primary is-light">{ "Today's Leader" }</button>
                <button onclick={shuffle} class="button is-link is-light">{ "Shuffle order" }</button>
            </div>
        </div>
    }
}
