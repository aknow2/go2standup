use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::*;
use rand::seq::SliceRandom;

#[derive(Clone, PartialEq)]
struct Member {
    id: usize,
    name: String,
}

#[derive(Properties, PartialEq)]
struct MembersListProps {
    members: Vec<Member>,
    on_remove: Callback<Member>
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

    let members: UseStateHandle<Vec<Member>> = use_state(Vec::new);
    let attended_members: UseStateHandle<Vec<Member>> = use_state(Vec::new);
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let meeting_status = use_state(|| Status::Preparing);
    let on_change_member_name = {
        let member_name = new_member_name.clone();
        Callback::from(move |e: Event| {
            let target = e.target().expect("Event should have a target when dispatched");
            let val = target.unchecked_into::<HtmlInputElement>().value();
            member_name.set(val)
        })
    };
    let add_member = {
        let members = members.clone();
        let new_member_name = new_member_name.clone();
        Callback::from(move |_| {
            let mut vec_members = members.to_vec();
            vec_members.push(Member {
                id: members.len(),
                name: new_member_name.to_string(),
            });
            members.set(vec_members);
            new_member_name.set(String::from(""));
            log::info!("Update: {:?} {:?}", new_member_name.to_string(), members.len());
        })
    };
    let clear_member = {
        let members = members.clone();
        Callback::from(move |member: Member| {
            let mut vecm = members.to_vec();
            let index = vecm.iter().position(|x| x.id == member.id).unwrap();
            vecm.remove(index);
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
                                                onchange={on_change_member_name}
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
                        <div class="is-size-4">{ "Parking lot" }</div>
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
