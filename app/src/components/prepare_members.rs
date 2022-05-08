use stylist::{ style };
use yew::prelude::*;
use web_sys::{HtmlInputElement};
use wasm_bindgen::*;
use crate::ctx::meeting::{MeetingActions, MeetingContext};
use crate::ctx::styles::StyleContext;
use crate::data;
use crate::components::member_list::MembersList;

#[function_component(PrepareMembers)]
pub fn prepare_members() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    let style_ctx = use_context::<StyleContext>().expect("no ctx found");
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

    let update_member = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |member: data::meeting::Member| {
            ctx.dispatch(MeetingActions::UpdateMember(member));
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

    let input = use_state(|| {
        let style = style!(
            r#"
                background-color: #1D3249;
                width: 95%;
                border: 0px;
                outline: none;
                padding: 0px;
                height: 100%;
                *:focus {
                    border: 0px;
                    outline: none;
                }
            "#
        ).expect("Failed to create style");

        style.get_class_name().to_string()
    });
    let text_container = use_state(|| {
        let style = style!(
            r#"
                background-color: #1D3249;
                padding: 0 4px;
                margin: 0px;
                width: 100%;
                height: 100%;
                border: 1px solid #aaa;
                border-radius: 4px;
                display: flex;
                align-items: center;
            "#
        ).expect("Failed to create style");
        style.get_class_name().to_string()
    });
    let container = use_state(|| {
        let style = style!(
            r#"
                display: flex;
                margin-bottom: 16px;
            "#
        ).expect("Failed to create style");
        style.get_class_name().to_string()
    });
    let button_group = use_state(|| {
        let style = style!(
            r#"
                padding: 0 8px;
                display: flex;
                gap: 0 16px;
            "#
        ).expect("Failed to create style");
        style.get_class_name().to_string()
    });
    let percent60w = use_state(|| {
        let style = style!(
            r#"
                width: 60%;
            "#
        ).expect("Failed to create style");
        style.get_class_name().to_string()
    });
    html!{
        <div>
            <div class={container.to_string()}>
                <div class={percent60w.to_string()}>
                    <div class={text_container.to_string()}>
                        <input
                            class={input.to_string()}
                            type="text"
                            placeholder="name"
                            value={new_member_name.to_string()}
                            onkeydown={keydown}
                            oninput={change_new_member_name}
                        />
                        <button class={style_ctx.icon_btn.to_string()}>
                            <span class="icon" onclick={add_member}>
                                <i class="material-icons">{"add"}</i>
                            </span>
                        </button>
                    </div>
                </div>
                <div class={button_group.to_string()}>
                    <button onclick={new_leader} class={style_ctx.outline_btn.to_string()}>{ "Today's Leader" }</button>
                    <button onclick={shuffle_members} class={style_ctx.outline_btn.to_string()}>{ "Shuffle" }</button>
                </div>
            </div>
            <div class={style_ctx.member_list.to_string()}>
                <MembersList
                    leader_id={leader_id.clone()}
                    members={members.to_vec()}
                    on_remove={remove_member}
                    on_update_member={update_member}
                />
            </div>
        </div>
    }
}
