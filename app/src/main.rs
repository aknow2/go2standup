mod data;
mod components;
mod repository;
mod ctx;
use yew::prelude::*;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use ctx::meeting::{MeetingProvider};

#[function_component(App)]
fn app() -> Html {
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
