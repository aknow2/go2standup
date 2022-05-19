mod data;
mod components;
mod repository;
mod ctx;
use stylist::style;
use yew::prelude::*;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use ctx::styles::{StyleProvider};
use ctx::meeting::{MeetingProvider, MeetingContext, MeetingStatus};

#[function_component(HeroLoading)]
fn hero_loading() -> Html {
    let container = style!(r#"
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    "#).expect("Failed to convert css").get_class_name().to_string();
    html!(
        <div class={container}>
            <div class="loader"></div>
        </div>
    )
}

#[function_component(MainContents)]
fn main_contents() -> Html {
    let root = style!(r#"
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
    "#).expect("").get_class_name().to_string();
    let container = style!(r#"
        display: flex;
        width: 100%;
        height: 90%;
    "#).expect("").get_class_name().to_string();
    let left_panel = style!(r#"
        padding-left: 32px;
        width: 70%;
        height: 100%;
    "#).expect("").get_class_name().to_string();
    let right_panel = style!(r#"
        width: 30%;
        min-width: 320px;
        height: 100%;
    "#).expect("").get_class_name().to_string();
    html! {
        <div class={root}>
            <div>
                <Header />
            </div>
            <div class={container}>
                <div class={left_panel}>
                    <PrepareMembers/>
                </div>
                <div class={right_panel}>
                    <ParkingLot />
                </div>
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");

    match meeting_ctx.meeting_status() {
        MeetingStatus::Initializing => html! { <HeroLoading /> }, 
        MeetingStatus::Ready => html!{ <MainContents /> },
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <StyleProvider>
            <MeetingProvider>
                <App></App>
            </MeetingProvider>
        </StyleProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}
