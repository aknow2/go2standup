use yew::prelude::*;
use stylist::{ style };

#[derive(Debug, PartialEq, Clone)]
pub struct StyleContext {
    pub header: String,
    pub member_card: String,
    pub member_list: String,
    pub outline_btn: String,
    pub icon_btn: String,
    pub flat_btn: String,
}

#[derive(Debug, PartialEq, Clone)]
struct StyleProviderState {
    ctx: StyleContext,
}

#[derive(Properties, Debug, PartialEq)]
pub struct StyleProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(StyleProvider)]
pub fn style_provider(props: &StyleProviderProps) -> Html {
    let state = use_state(|| {
        // Card
        let member_card_style = style!(
            r#"
               width: 190px;
               height: 158px;
               border-radius: 5px;
               background: #272D44;
               box-shadow:  1px 1px 2px #171b28,
                            -1px -1px 2px #373f60;
            "#
         ).expect("Failed to mount style");
        let member_card = member_card_style.get_class_name().to_string();

         // Member list
        let member_list_style = style!(
            r#"
               display: flex;
               gap: 24px;
               flex-wrap: wrap;
            "#
         ).expect("Failed to mount style");
        let member_list = member_list_style.get_class_name().to_string();

        // header
        let header_style = style!(
            r#"
               display: flex;
               box-sizing: border-box;
               width: 100%;
               justify-content: space-between;
               align-items: center;
               height: 48px;
               padding: 16px;
               margin-bottom: 16px;
            "#
         ).expect("Failed to mount style");
        let header = header_style.get_class_name().to_string();

        let icon_btn_style = style!(
            r#"
                padding: 0px;
                border: 0px;
                background: none;
                border-radius: 50%;
                outline: none;
                display: flex;
                justify-content: center;
                align-items: center;
                *:active {
                    background-color: rgba(230, 230, 230, 0.5);
                    border-radius: 50%;
                    border: 0px;
                    outline: none;
                }
            "#
        ).expect("Failed to create style");
        let icon_btn = icon_btn_style.get_class_name().to_string();
        let flat_btn_style = style!(
            r#"
                width: 100%;
                height: 100%;
                padding: 0px;
                border: 0px;
                background: none;
                outline: none;
                display: flex;
                justify-content: center;
                align-items: center;
                &:active {
                    background-color: rgba(230, 230, 230, 0.5);
                    border: 0px;
                    outline: none;
                }
            "#
        ).expect("Failed to create style");
        let flat_btn = flat_btn_style.get_class_name().to_string();

        let outline_btn_style = style!(
            r#"
                padding: 4px;
                border: 1px solid #03A688;
                color: #03A688;
                border-radius: 5px;
                background: none;
                outline: none;
                display: flex;
                justify-content: center;
                align-items: center;
                &:active {
                    border: 1px solid #038C7F;
                    color: #038C7F;
                }
                &.info {
                    border: 1px solid #9BDAF2;
                    color: #9BDAF2;
                }
                &.secondary {
                    border: 1px solid #818274;
                    color: #818274;
                }
            "#
        ).expect("Failed to create style");
        let outline_btn = outline_btn_style.get_class_name().to_string();

        StyleProviderState {
            ctx: StyleContext {
                flat_btn,
                member_card,
                member_list,
                header,
                outline_btn,
                icon_btn,
            },
        }
    });

    html! {
        <ContextProvider<StyleContext> context={state.ctx.clone()}>
            {props.children.clone()}
        </ContextProvider<StyleContext>>
    }
}
