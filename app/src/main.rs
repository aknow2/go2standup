mod data;
mod components;
mod repository;

use yew::prelude::*;
use rand::prelude::SliceRandom;
use repository::local_repository::Repository;
use components::prepare_members:: { PrepareMembers };
use components::meeting:: { Meeting };
use components::header:: { Header };

enum Status {
    Preparing,
    Meeting,
}

#[function_component(App)]
fn app() -> Html {
    let repository: UseStateHandle<Option<Repository>> = use_state(|| None);
    let members: UseStateHandle<data::member::Members> = use_state(Vec::new);
    let attended_members: UseStateHandle<data::member::Members> = use_state(Vec::new);
    let new_member_name: UseStateHandle<String>= use_state(|| String::from(""));
    let memo: UseStateHandle<String>= use_state(|| String::from(""));
    let meeting_status = use_state(|| Status::Preparing);
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
            id: members.len(),
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

    let start_meeting = {
        let status = meeting_status.clone();
        let members = members.clone();
        let attended_members = attended_members;

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

    let content = match *meeting_status {
        Status::Preparing => html! {
            <PrepareMembers
                members={members.to_vec()}
                new_member_name={new_member_name.to_string()}
                on_change_new_member_name={on_change_member_name}
                on_remove={remove_member}
                on_add_member={add_member}
                on_start_meeting={start_meeting}
            />
        },
        Status::Meeting => html! {
            <Meeting 
                members={members.to_vec()}
                on_back={back}
                memo={memo.to_string()}
                on_change_memo={on_change_memo}
            />
        },
    };

    html! {
        <div>
            <Header />
            { content }
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
