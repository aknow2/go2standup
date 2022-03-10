use web_sys::{HtmlInputElement, window};
use yew::prelude::*;
use wasm_bindgen::*;
use rand::seq::SliceRandom;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Member {
    id: usize,
    name: String,
}

type Members = Vec<Member>;

#[derive(Properties, PartialEq)]
struct MembersListProps {
    members: Members,
    on_remove: Callback<Member>
}

fn save_members(members: &Members) {
    let window = window().unwrap();
    if let Ok(Some(storage)) =  window.local_storage() {
        let text = serde_json::to_string(&members).unwrap();
        log::info!("Result: {:?}", text);
        storage.set_item("members", &text).unwrap();
    } 
}

fn fetch_members() -> Members {
    let window = window().unwrap();
    if let Ok(Some(storage)) =  window.local_storage() {
        let text = storage.get_item("members").unwrap().unwrap_or_else(|| String::from("[]"));
        let result = serde_json::from_str::<Members>(&text).expect("");
        log::info!("Result: {:?}", result);
        result
    } else {
        vec![]
    }
}

#[function_component(MembersList)]
fn members_list(MembersListProps { members, on_remove }: &MembersListProps) -> Html {
    members.iter().map(|member| {
        let  on_remove_member = {
            let on_remove = on_remove.clone();
            let mem = member.clone();
            Callback::from(move |_| {
                on_remove.emit(mem.clone())
            })
        };
        html!{
            <div class="columns is-vcentered">
                <div class="column is-four-fifths is-size-3">
                    {member.name.to_string()}
                </div>
                <div class="column is-half">
                    <div class="button is-white" onclick={on_remove_member}>
                        <span class="icon is-large">
                            <i class="material-icons">{"close"}</i>
                        </span>
                    </div>
                </div>
            </div>
        }
    }).collect::<Html>()
}

enum Status {
    Preparing,
    Meeting,
}

#[function_component(App)]
fn app() -> Html {
    let members: UseStateHandle<Members> = use_state(fetch_members);
    let attended_members: UseStateHandle<Members> = use_state(Vec::new);
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let meeting_status = use_state(|| Status::Preparing);
    let on_change_member_name = {
        let member_name = new_member_name.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().expect("Event should have a target when dispatched");
            let val = target.unchecked_into::<HtmlInputElement>().value();
            log::info!("Update new member name: {:?}", val);
            member_name.set(val);
        })
    };

    let update_members = |
            members: &UseStateHandle<Members>,
            new_member_name: &UseStateHandle<String>
        | {
        let mut vec_members = members.to_vec();
        vec_members.push(Member {
            id: members.len(),
            name: new_member_name.to_string(),
        });
        save_members(&vec_members);
        members.set(vec_members);
        new_member_name.set(String::from(""));
        log::info!("Update: {:?} {:?}", new_member_name.to_string(), members.len());
    };

    let add_member = {
        let members = members.clone();
        let new_member_name = new_member_name.clone();
        Callback::from(move |_| {
            update_members(&members, &new_member_name);
        })
    };
    let clear_member = {
        let members = members.clone();
        Callback::from(move |member: Member| {
            let mut vecm = members.to_vec();
            let index = vecm.iter().position(|x| x.id == member.id).unwrap();
            vecm.remove(index);
            save_members(&vecm);
            members.set(vecm);
        })
    };
    let start_meeting = {
        let status = meeting_status.clone();
        let members = members.clone();
        let attended_members = attended_members.clone();

        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            let mut member_list = members.to_vec();
            member_list.shuffle(&mut rng);
            status.set(Status::Meeting);
            attended_members.set(member_list);
        })
    };
    let back = {
        let status = meeting_status.clone();

        Callback::from(move |_| {
            status.set(Status::Preparing);
        })
    };
    let on_keydown = {
        let members = members.clone();
        let new_member_name = new_member_name.clone();
        Callback::from(move |e: KeyboardEvent| {
            log::info!("on key down {:?}", e.key_code());
            if e.key_code() == 13 {
                update_members(&members, &new_member_name);
            }
        })
    };

    match *meeting_status {
        Status::Preparing => html! {
            <div class="container is-max-widescreen">
                <div class="columns  is-centered">
                    <div class="column is-half">
                        <div class="card mt-5">
                            <div class="card-content ">
                                <span class="is-size-2">{ "Members" }</span>
                                <div class="py-6">
                                    <MembersList members={members.to_vec()} on_remove={clear_member} ></MembersList>
                                </div>
                                <div>
                                    <div class="columns is-vcentered">
                                        <div class="column is-four-fifths">
                                            <input
                                                class="input is-large"
                                                type="text"
                                                placeholder="name"
                                                value={new_member_name.to_string()}
                                                onkeydown={on_keydown}
                                                oninput={on_change_member_name}
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
                                    <div onclick={start_meeting} class="button is-success">{ "Start" }</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        },
        Status::Meeting => html! {
            <div class="container is-max-widescreen py-3">
                <div class="columns">
                    <div class="column">
                        <div class="button is-white" onclick={back}>
                            <span class="icon is-large">
                                <i class="material-icons">{"undo"}</i>
                            </span>
                        </div>
                    </div>
                </div>
                <div class="columns">
                    <div class="column">
                        {
                            attended_members.iter().map(|member| {
                                html! {
                                    <div class="card mt-6 px-3">
                                        <div class="columns is-vcentered">
                                            <div class="column is-size-3">
                                                {member.name.to_string()}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="column">
                        <div class="is-size-4">{ "Meeting memo" }</div>
                        <textarea class="textarea" rows="10"></textarea>
                    </div>
                </div>
            </div>
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
