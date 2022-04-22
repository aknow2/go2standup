use yew::prelude::*;
use crate::data;
use crate::components::member_card::MemberCard;

#[derive(Properties, PartialEq)]
pub struct MembersListProps {
    pub members: data::meeting::Members,
    pub on_remove: Callback<data::meeting::Member>,
    pub leader_id: Option<String>,
}

#[function_component(MembersList)]
pub fn members_list(MembersListProps { leader_id, members, on_remove }: &MembersListProps) -> Html {
    members.iter().enumerate().map(|(i, member)| {
        let is_leader = match leader_id {
            Some(id) => *id == member.id,
            None => false,
        };
        let on_remove = on_remove.clone();
        html!{
            <MemberCard
                member={member.clone()}
                order={i+1}
                is_leader={is_leader}
                on_remove={on_remove}
            />
        }
    }).collect::<Html>()
}
