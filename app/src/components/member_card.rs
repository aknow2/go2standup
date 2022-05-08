use stylist::style;
use yew::prelude::*;
use crate::data::meeting::ReactionType;
use crate::{data, ctx::styles::StyleContext};
use crate::components::typography::{Typography, TextSize};

#[derive(PartialEq)]
pub enum Flip {
    Front,
    Back
}

#[derive(Properties, PartialEq)]
pub struct FrontProps {
    pub member: data::meeting::Member,
    pub on_remove: Callback<data::meeting::Member>,
    pub on_flip: Callback<Flip>,
    pub is_leader: bool,
    pub order: usize
}

fn emoji(reactionType: ReactionType) -> &'static str {
    match reactionType {
        ReactionType::Thumbup => "\u{1F44D}",
        ReactionType::Thumbdown => "\u{1F44E}",
        ReactionType::Clap => "\u{1F44E}",
        ReactionType::Smile => "\u{1F642}",
        ReactionType::I => "\u{0031}",
        ReactionType::II => "\u{0032}",
        ReactionType::III => "\u{0033}",
        ReactionType::IV => "\u{0034}",
        ReactionType::V => "\u{0035}",
        ReactionType::VI => "\u{0036}",
        ReactionType::VII => "\u{0037}",
        ReactionType::VIII => "\u{0038}",
        ReactionType::IX => "\u{0039}",
        ReactionType::X => "\u{1F51F}",
        _ => "\u{1F610}"
    }
}

#[function_component(Front)]
pub fn front(FrontProps { is_leader, member, on_remove, order, on_flip }: &FrontProps) -> Html {
    let style_ctx = use_context::<StyleContext>().expect("no ctx found");
    let on_remove_member = {
        let on_remove = on_remove.clone();
        let mem = member.clone();
        Callback::from(move |_| {
            on_remove.emit(mem.clone())
        })
    };

    let flip_to_back = {
        let on_flip = on_flip.clone();
        Callback::from(move |_| {
            on_flip.emit(Flip::Back)
        })
    };

    let header_content = match is_leader {
        true => html! {
            <span class="icon has-text-success">
                <i class="material-icons">{"flag"}</i>
            </span> },
        false => html! {<span class="is-size-5 pl-1">{ order }</span>},
    };
    let card_header = use_state(|| {
        let s = style!(
            r#"
                padding: 4px;
                width: 100%;
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#
        ).expect("failed to convert css");
        s.get_class_name().to_string()
    });
    let card_footer = use_state(|| {
        let s = style!(
            r#"
                padding: 4px;
                width: 100%;
                display: flex;
                justify-content: center;
                align-items: center;
            "#
        ).expect("failed to convert css");
        s.get_class_name().to_string()
    });
    html!{
        <div>
            <div class={&*card_header}>
                <div>
                    {header_content}       
                </div>
                <div>
                    <button class={style_ctx.icon_btn.to_string()} onclick={on_remove_member}>
                        <span>
                            <i class="material-icons">{"clear"}</i>
                        </span>
                    </button>
                </div>
            </div>
            <div>
                <button class={style_ctx.flat_btn.to_string()} onclick={flip_to_back}>
                    <Typography size={TextSize::H1}>
                    {"\u{1F378}"}
                    </Typography>
                </button>
            </div>
            <div class={&*card_footer}>
                {&member.name}
            </div>
        </div>
    }
}


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
    let flip =  use_state(|| Flip::Front);
    let on_flip = {
        let flip = flip.clone();
        Callback::from(move |dir| {
            flip.set(dir);
        })
    };
    html!{
        <div class={style_ctx.member_card.to_string()}>
            <Front 
                is_leader={is_leader.clone()}
                on_remove={on_remove}
                on_flip={on_flip}
                member={member.clone()}
                order={order.clone()}
            />
        </div>
    }
}
