mod data;
mod components;
mod repository;
mod ctx;
use yew::prelude::*;
use components::prepare_members:: { PrepareMembers };
use components::parking_lot:: { ParkingLot };
use components::header:: { Header };
use ctx::meeting::{MeetingProvider, MeetingContext, MeetingActions};

#[function_component(App)]
fn app() -> Html {
    let meeting_ctx = use_context::<MeetingContext>().expect("no ctx found");
    log::info!("{:?}", meeting_ctx.state);

    {
        let ctx = meeting_ctx.clone();
        use_effect_with_deps(
            move |_| {
                {
                    let search = web_sys::window().unwrap().location().search().unwrap();
                    let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
                    let id = params.get("id");
                    log::info!("Search: {:?}", id);
                    ctx.dispatch(MeetingActions::StartMeeting(id));
                }
                || ()
            },
            (),
        );
    }


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
