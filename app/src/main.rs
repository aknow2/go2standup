mod data;
mod components;
mod repository;
mod ctx;
use yew::prelude::*;
use rand::prelude::SliceRandom;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use gloo_timers::callback::Timeout;
use ctx::meeting::{MeetingProvider, MeetingContext, MeetingActions};
use crate::repository::api::subscribe_meeting;

#[function_component(App)]
fn app() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    log::info!("{:?}", meeting_ctx.state);

    let state = meeting_ctx.state.clone();
    let members = state.members.to_vec();
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let memo = state.memo.to_string();
    let leader_id: UseStateHandle<Option<String>>= use_state(|| None);
    let loot_time = use_state(|| u32::MAX);

    {
        let leader_id = leader_id.clone();
        let members = members.clone();
        let loot_time = loot_time.clone();
        use_effect(move || {
                log::info!("Looting");
                let leader_id = leader_id.clone();
                let members = members.clone();
                Timeout::new(*loot_time, move || {
                    log::info!("Timeout {:?}", members.len());
                    if *loot_time < 420 {
                        let mut rng = rand::thread_rng();
                        let mut member_list = members.to_vec();
                        member_list.shuffle(&mut rng);

                        if let Some(member) = member_list.pop() {
                            log::info!("Leader id: {:?}", member.id);
                            leader_id.set(Some(member.id));
                        }

                        let time = ((*loot_time as f64) * 1.1) as u32;
                        loot_time.set(time);
                    }
                }).forget();
                || ()
            }
        );
    }

    let on_change_member_name = {
        let member_name = new_member_name.clone();
        Callback::from(move |val: String| {
            member_name.set(val);
        })
    };
    let on_change_memo = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |memo| {
            ctx.dispatch(MeetingActions::UpdateMemo(memo));
        })
    };
    {
        let ctx = meeting_ctx.clone();
        use_effect_with_deps(
            move |_| {
                {
                    let search = web_sys::window().unwrap().location().search().unwrap();
                    let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
                    let id = params.get("id");
                    log::info!("Search: {:?}", id);
                    ctx.dispatch(MeetingActions::StartMeeting(id));
                }
                || ()
            },
            (),
        );
    }

    let add_member = {
        let new_member_name = new_member_name.clone();
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::AddMember(new_member_name.to_string()));
            new_member_name.set(String::from(""))
        })
    };

    let remove_member = {
        let members = members.clone();
        let ctx = meeting_ctx.clone();
        Callback::from(move |member: data::meeting::Member| {
            ctx.dispatch(MeetingActions::RemoveMember(member.id.to_string()));
            let mut vecm = members.to_vec();
            let index = vecm.iter().position(|x| x.id == member.id).unwrap();
            vecm.remove(index);
        })
    };

    let shuffle_members = {
        let members = members.clone();
        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            let mut member_list = members.to_vec();
            member_list.shuffle(&mut rng);
            log::info!("Shuffle: {:?}", member_list.to_vec());
            // members.set(member_list);
        })
    };

    let loot_leader = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::LootLeader);
            log::info!("Start loot ");
            loot_time.set(10);
        })
    };

    html! {
        <div>
            <Header />
            <div class="container is-max-widescreen">
                <div class="columns  is-centered my-2">
                    <div class="column">
                        <PrepareMembers
                            leader_id={(*leader_id).clone()}
                            members={members.to_vec()}
                            new_member_name={new_member_name.to_string()}
                            on_change_new_member_name={on_change_member_name}
                            on_remove={remove_member}
                            on_add_member={add_member}
                            on_shuffle={shuffle_members}
                            on_loot_leader={loot_leader}
                        />
                    </div>
                    <div class="column">
                        <ParkingLot
                            memo={memo}
                            on_change_memo={on_change_memo}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <MeetingProvider>
            <App></App>
        </MeetingProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    subscribe_meeting("88c08943-6f66-47dd-8138-416f840b8229");
    yew::start_app::<Root>();
}
