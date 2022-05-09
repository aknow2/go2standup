use stylist::style;
use yew::prelude::*;
use crate::data::meeting::{ReactionType, Member};
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

fn emoji(reactionType: &ReactionType) -> &'static str {
    match reactionType {
        ReactionType::THUMBSUP => "\u{1F44D}",
        ReactionType::THUMBSDOWN => "\u{1F44E}",
        ReactionType::CLAP => "\u{1F44F}",
        ReactionType::SMILE => "\u{1F642}",
        ReactionType::ZERO => "\u{0030}\u{fe0f}\u{20e3}",
        ReactionType::I => "\u{0031}\u{fe0f}\u{20e3}",
        ReactionType::II => "\u{0032}\u{fe0f}\u{20e3}",
        ReactionType::III => "\u{0033}\u{fe0f}\u{20e3}",
        ReactionType::IV => "\u{0034}\u{fe0f}\u{20e3}",
        ReactionType::V => "\u{0035}\u{fe0f}\u{20e3}",
        ReactionType::VI => "\u{0036}\u{fe0f}\u{20e3}",
        ReactionType::VII => "\u{0037}\u{fe0f}\u{20e3}",
        ReactionType::VIII => "\u{0038}\u{fe0f}\u{20e3}",
        ReactionType::IX => "\u{0039}\u{fe0f}\u{20e3}",
        ReactionType::X => "\u{1F51F}",
        ReactionType::NONE => "\u{1F610}"
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
            <Typography size={TextSize::H3}>
                <i class="material-icons">{"flag"}</i>
            </Typography> },
        false => html! {<Typography size={TextSize::H3}>{ order }</Typography>},
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
                .item {
                    text-align: center;
                    min-width: 100%;
                    text-overflow: ellipsis;
                    overflow: hidden;
                }
            "#
        ).expect("failed to convert css");
        s.get_class_name().to_string()
    });
    let emoji = emoji(&member.reaction);
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
                    {emoji}
                    </Typography>
                </button>
            </div>
            <div class={&*card_footer}>
                <div class="item">
                    <Typography size={TextSize::H4}>
                        {&member.name}
                    </Typography>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BackProps {
    pub on_flip: Callback<Flip>,
    pub on_select_reaction: Callback<ReactionType>,
}

#[function_component(Back)]
pub fn back(BackProps { on_flip, on_select_reaction }: &BackProps) -> Html {
    let style_ctx = use_context::<StyleContext>().expect("no ctx found");

    let flip_to_front = {
        let on_flip = on_flip.clone();
        Callback::from(move |_| {
            on_flip.emit(Flip::Front)
        })
    };


    let emojis: Vec<Html> = ReactionType::itr().map(|reaction| {
        let select_reaction = {
            let on_select_reaction = on_select_reaction.clone();
            Callback::from(move |_| {
                on_select_reaction.emit(reaction.clone())
            })
        };
        html!(
            <button class={style_ctx.icon_btn.to_string()} onclick={select_reaction}>
                <Typography size={TextSize::H4}>{emoji(reaction)}</Typography>
            </button>
        )
    }).collect();

    let card_header = use_state(|| {
        let s = style!(
            r#"
                padding: 4px;
                width: 100%;
                display: flex;
                align-items: center;
            "#
        ).expect("failed to convert css");
        s.get_class_name().to_string()
    });
    let content = use_state(|| {
        let s = style!(
            r#"
                padding: 4px;
                gap: 8px;
                width: 100%;
                height: 110px;
                overflow-y: scroll;
                display: flex;
                flex-wrap: wrap;
                justify-content: space-between;
            "#
        ).expect("failed to convert css");
        s.get_class_name().to_string()
    });
    html!{
        <div>
            <div class={&*card_header}>
                <button class={style_ctx.icon_btn.to_string()} onclick={flip_to_front}>
                    <i class="material-icons">{"arrow_back"}</i>
                </button>
            </div>
            <div class={&*content}>
                { emojis }
            </div>
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct MemberCardProps {
    pub member: data::meeting::Member,
    pub on_update_member: Callback<data::meeting::Member>,
    pub on_remove: Callback<data::meeting::Member>,
    pub is_leader: bool,
    pub order: usize
}

#[function_component(MemberCard)]
pub fn members_card(MemberCardProps { is_leader, member, on_remove, order, on_update_member }: &MemberCardProps) -> Html {
    let style_ctx = use_context::<StyleContext>().expect("no ctx found");
    let flip =  use_state(|| Flip::Front);
    let on_flip = {
        let flip = flip.clone();
        Callback::from(move |dir| {
            flip.set(dir);
        })
    };
    let on_update_reaction = {
        let update = on_update_member.clone();
        let member = member.clone();
        let flip = flip.clone();
        Callback::from(move |reaction| {
            flip.set(Flip::Front);
            update.emit(
                Member {
                    reaction,
                    ..(member.clone())
                }
            )
        })
    };
    html!{
        <div class={style_ctx.member_card.to_string()}>
            {
                match &*flip {
                    Flip::Front => html!{<Front 
                        is_leader={is_leader.clone()}
                        on_remove={on_remove}
                        on_flip={on_flip}
                        member={member.clone()}
                        order={order.clone()}
                    />},
                    Flip::Back => html!{<Back
                            on_flip={on_flip}
                            on_select_reaction={on_update_reaction}
                        />},
                }
            }
            
        </div>
    }
}
