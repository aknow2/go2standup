use yew::prelude::*;
use stylist::{ Style };

#[derive(Debug, PartialEq)]
pub enum TextSize {
  H1,
  H2,
  H3,
  H4,
  Body,
}

#[derive(Properties, Debug, PartialEq)]
pub struct TypographyProps {
    #[prop_or_default]
    pub children: Children,
    pub size: TextSize,
}

#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let text_style = use_state(|| {
      let size = match props.size {
        TextSize::H1 => "48px",
        TextSize::H2 => "36px",
        TextSize::H3 => "28px",
        TextSize::H4 => "24px",
        TextSize::Body => "18px",
      };
      let str = format!("font-size: {};", size);
      Style::new(str).expect("failed to convert css").get_class_name().to_string()
    }); 

    html! {
        <span class={text_style.to_string()}>
            {props.children.clone()}
        </span>
    }
}

