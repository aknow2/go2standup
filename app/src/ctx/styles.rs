use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

struct StyleContext {

}

#[function_component(StyleProvider)]
pub fn styleProvider_provider() -> Html {
    html! {
        <ContextProvider<StyleContext> context={StyleContext{}}>
            {props.children.clone()}
        </ContextProvider<StyleContext>>
    }
}

