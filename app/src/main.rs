mod data;
mod components;
mod repository;
mod ctx;
use yew::prelude::*;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use ctx::meeting::{MeetingProvider, MeetingContext, MeetingActions};

#[function_component(App)]
fn app() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    log::info!("{:?}", meeting_ctx.state);

    let state = meeting_ctx.state.clone();
    let members = state.members.to_vec();
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let memo = state.memo.to_string();
    let leader_id = state.leader_id.clone();
    log::info!("This leader! {:?}", leader_id);

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
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::ShuffleMembers);
        })
    };

    let loot_leader = {
        let ctx = meeting_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(MeetingActions::NewLeader);
            log::info!("Start loot ");
        })
    };

    html! {
        <div>
            <Header />
            <div class="container is-max-widescreen">
                <div class="columns  is-centered my-2">
                    <div class="column">
                        <PrepareMembers
                            leader_id={leader_id.clone()}
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
    yew::start_app::<Root>();
}
