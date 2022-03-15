use yew::prelude::*;
use crate::data;

#[derive(Properties, PartialEq)]
pub struct MembersListProps {
    pub members: data::member::Members,
    pub on_remove: Callback<data::member::Member>
}

#[function_component(MembersList)]
pub fn members_list(MembersListProps { members, on_remove }: &MembersListProps) -> Html {
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
