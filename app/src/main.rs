mod data;
mod components;
mod repository;

use yew::prelude::*;
use rand::prelude::SliceRandom;
use repository::local_repository::Repository;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use gloo_timers::callback::Timeout;

#[function_component(App)]
fn app() -> Html {
    let repository: UseStateHandle<Option<Repository>> = use_state(|| None);
    let members: UseStateHandle<data::member::Members> = use_state(Vec::new);
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let memo: UseStateHandle<String>= use_state(|| String::from(""));
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

    {
        let repository = repository.clone();
        let members = members.clone();
        use_effect_with_deps(
            move |members| {
                if let Some(repo) = &*repository {
                    repo.save_members(members);
                }
                ||()
            },
            members,
        )
    }

    {
        let repository = repository.clone();
        let memo = memo.clone();
        use_effect_with_deps(
            move |memo| {
                if let Some(repo) = &*repository {
                    repo.save_memo(&*memo);
                }
                || ()
            },
            memo,
        );
    }

    let on_change_member_name = {
        let member_name = new_member_name.clone();
        Callback::from(move |val: String| {
            member_name.set(val);
        })
    };
    let on_change_memo = {
        let memo = memo.clone();
        Callback::from(move |val| {
            memo.set(val);
        })
    };


    {
        let members = members.clone();
        let memo = memo.clone();
        use_effect_with_deps(
            move |_| {
                let repo = Repository::new();
                if let Some(repo) = repo {
                    members.set(repo.fetch_members());
                    memo.set(repo.fetch_memo());
                    repository.set(Some(repo));
                }
                || ()
            },
            (),
        );
    }

    let update_members = |
            members: &UseStateHandle<data::member::Members>,
            new_member_name: &UseStateHandle<String>
        | {
        let mut vec_members = members.to_vec();
        vec_members.push(data::member::Member {
            id: uuid::Uuid::new_v4().to_string(),
            name: new_member_name.to_string(),
        });
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

    let remove_member = {
        let members = members.clone();
        Callback::from(move |member: data::member::Member| {
            let mut vecm = members.to_vec();
            let index = vecm.iter().position(|x| x.id == member.id).unwrap();
            vecm.remove(index);
            members.set(vecm);
        })
    };

    let shuffle_members = {
        let members = members.clone();
        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            let mut member_list = members.to_vec();
            member_list.shuffle(&mut rng);
            log::info!("Shuffle: {:?}", member_list.to_vec());
            members.set(member_list);
        })
    };

    let loot_leader = {
        Callback::from(move |_| {
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
                            memo={memo.to_string()}
                            on_change_memo={on_change_memo}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
