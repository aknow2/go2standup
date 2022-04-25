use yew::prelude::*;
use crate::{data, ctx::styles::StyleContext};

#[derive(Properties, PartialEq)]
pub struct MemberCardProps {
    pub member: data::meeting::Member,
    pub on_remove: Callback<data::meeting::Member>,
    pub is_leader: bool,
    pub order: usize
}

#[function_component(MemberCard)]
pub fn members_card(MemberCardProps { is_leader, member, on_remove, order }: &MemberCardProps) -> Html {
    let style_ctx = use_context::<StyleContext>().expect("no ctx found");
    
    let on_remove_member = {
        let on_remove = on_remove.clone();
        let mem = member.clone();
        Callback::from(move |_| {
            on_remove.emit(mem.clone())
        })
    };

    let header_content = match is_leader {
        true => html! {
            <span class="icon has-text-success">
                <i class="material-icons">{"flag"}</i>
            </span> },
        false => html! {<span class="is-size-5 pl-1">{ order }</span>},
    };

    html!{
        <div class={style_ctx.card.to_string()}>
            <div class="level mb-0">
                <div class="level-left">
                    <div class="level-item ml-2">
                        {header_content}       
                    </div>
                </div>
                <div class="level-right">
                    <div class="level-item">
                        <button class="card-header-icon" onclick={on_remove_member}>
                            <span class="icon">
                                <i class="material-icons">{"clear"}</i>
                            </span>
                        </button>
                    </div>
                </div>
            </div>
            <div class="block is-flex is-justify-content-center mb-0">
                <span class="is-size-1">
                    {"\u{1F378}"}
                </span>
            </div>
            <footer class="card-footer">
                <div class="card-footer-item">
                    {&member.name}
                </div>
            </footer>
        </div>
    }
}
