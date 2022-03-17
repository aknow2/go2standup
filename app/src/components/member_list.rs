use yew::prelude::*;
use crate::data;

#[derive(Properties, PartialEq)]
pub struct MembersListProps {
    pub members: data::member::Members,
    pub on_remove: Callback<data::member::Member>,
    pub leader_id: Option<String>,
}

#[function_component(MembersList)]
pub fn members_list(MembersListProps { leader_id, members, on_remove }: &MembersListProps) -> Html {
    members.iter().enumerate().map(|(i, member)| {
        let  on_remove_member = {
            let on_remove = on_remove.clone();
            let mem = member.clone();
            Callback::from(move |_| {
                on_remove.emit(mem.clone())
            })
        };
        let leader_class = "notification is-primary block px-2 is-outlined";
        let member_class = "notification block px-2 is-outlined";

        let is_leader = match leader_id {
            Some(id) => *id == member.id,
            None => false,
        };
        let class_name = if is_leader { leader_class } else { member_class };

        let prepend_content = match is_leader {
            true => html! {
                <span class="icon">
                    <i class="material-icons">{"flag"}</i>
                </span> },
            false => html! {<span class="is-size-5 pl-1">{ i+1 }</span>},
        };

        html!{
            <div class={class_name}>
               <div class="columns is-vcentered">
                    <div class="column">
                        { prepend_content }
                    </div>
                    <div class="column is-four-fifths is-size-5">
                        {member.name.to_string()}
                    </div>
                    <div class="column">
                        <button class="delete" onclick={on_remove_member}></button>
                    </div>
                </div>
            </div>
        }
    }).collect::<Html>()
}
