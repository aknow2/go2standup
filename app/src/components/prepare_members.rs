use yew::prelude::*;
use web_sys::{HtmlInputElement};
use wasm_bindgen::*;
use crate::data;
use crate::components::member_list::MembersList;


#[derive(Properties, PartialEq)]
pub struct PrepareMembersProps {
    pub members: data::member::Members,
    pub new_member_name: String,
    pub on_change_new_member_name: Callback<String>,
    pub on_remove: Callback<data::member::Member>,
    pub on_add_member: Callback<()>,
    pub on_start_meeting: Callback<()>,
}

#[function_component(PrepareMembers)]
pub fn prepare_members(PrepareMembersProps {
        members,
        on_add_member,
        on_remove,
        new_member_name,
        on_change_new_member_name,
        on_start_meeting,
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
    let start_meeting = {
        let on_start_meeting = on_start_meeting.clone();
        Callback::from(move |_| {
            log::info!("on start meeting");
            on_start_meeting.emit(());
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
        <div class="container is-max-widescreen">
            <div class="columns  is-centered">
                <div class="column is-half">
                    <div class="card mt-5">
                        <div class="card-content ">
                            <span class="is-size-2">{ "Members" }</span>
                            <div class="py-6">
                                <MembersList members={members.to_vec()} on_remove={on_remove} />
                            </div>
                            <div>
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
                            </div>
                            <div class="mt-3">
                                <div onclick={start_meeting} class="button is-primary">{ "Start" }</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
