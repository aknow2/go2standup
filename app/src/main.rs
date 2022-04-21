mod data;
mod components;
mod repository;
mod ctx;
use yew::prelude::*;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use ctx::meeting::{MeetingProvider, MeetingContext, MeetingStatus};

#[function_component(HeroLoading)]
fn hero_loading() -> Html {
    html!(
        <section class="hero is-fullheight">
            <div class="hero-body">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-half">
                            <progress class="progress is-large is-primary" max="100">{"80%"}</progress>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    )
}

#[function_component(MainContents)]
fn main_contents() -> Html {
    html! {
        <div>
            <Header />
            <div class="container is-max-widescreen">
                <div class="columns  is-centered my-2">
                    <div class="column">
                        <PrepareMembers/>
                    </div>
                    <div class="column">
                        <ParkingLot />
                    </div>
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
        <MeetingProvider>
            <App></App>
        </MeetingProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}
