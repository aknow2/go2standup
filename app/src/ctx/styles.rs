use yew::prelude::*;
use stylist::{ style };

#[derive(Debug, PartialEq, Clone)]
pub struct StyleContext {
    pub header: String,
    pub card: String,
    pub member_list: String,
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
pub fn styleProvider_provider(props: &StyleProviderProps) -> Html {
    let state = use_state(|| {
        // Card
        let card_style = style!(
            r#"
               width: 190px;
               border: thin solid #cccccc;
               border-radius: 4px;
            "#
         ).expect("Failed to mount style");
        let card = card_style.get_class_name().to_string();

         // Member list
        let member_list_style = style!(
            r#"
               display: flex;
               gap: 16px;
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
               border-bottom: thin solid #333333;
            "#
         ).expect("Failed to mount style");
        let header = header_style.get_class_name().to_string();



        StyleProviderState {
            ctx: StyleContext {
                card,
                member_list,
                header,
            },
        }
    });
    html! {
        <ContextProvider<StyleContext> context={state.ctx.clone()}>
            {props.children.clone()}
        </ContextProvider<StyleContext>>
    }
}
