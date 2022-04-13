use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{data::meeting:: { Meeting, Member, ErrorMsg }, repository::{storage::{get_meeting_id, set_meeting_id}, api::{fetch_meeting, create_meeting}}};

pub enum MeetingActions {
    StartMeeting(Option<String>),
    UpdateMemo(String),
    AddMember(String),
    RemoveMember(String),
    LootLeader,
}

async fn start_meeting(query_id: Option<String>, current: &MeetingState) -> MeetingState {

    let meeting_id = match query_id {
        Some(qid) =>  Some(qid),
        None =>  get_meeting_id(),
    };

    let result: Result<Meeting, ErrorMsg> = match meeting_id {
        Some(mid) => fetch_meeting(mid).await,
        None => create_meeting().await,
    };

    match result {
        Ok(meeting) => {
            set_meeting_id(&meeting.id);
            MeetingState { 
                id: meeting.id,
                leader_id: meeting.leader_id,
                members: meeting.members,
                memo: meeting.memo,
                ..current.clone()
            }
        },
        Err(msg) => {
            MeetingState { 
                error_msg: Some(msg),
                ..current.clone()
            }
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MeetingState {
    pub id: String,
    pub leader_id: Option<String>,
    pub members: Vec<Member>,
    pub memo: String,
    pub error_msg: Option<ErrorMsg>
}

#[derive(Debug, PartialEq, Clone)]
pub struct MeetingContext {
   pub state: UseStateHandle<MeetingState>
}

impl MeetingContext {
    fn new(state: UseStateHandle<MeetingState>) -> MeetingContext {

        MeetingContext {
            state,
        }
    }

    pub fn dispatch(&self, action: MeetingActions) {
        let state = self.state.clone();
        spawn_local( async move  {
            match action {
                MeetingActions::StartMeeting(id) => {
                        let new_state = start_meeting(id, &state).await;
                        log::info!("start meeting {:?}", new_state);
                        state.set(new_state);
                },
                MeetingActions::AddMember(name) => {
                    log::info!("add member {:?}", name);
                },
                MeetingActions::RemoveMember(id) => {
                    log::info!("remove member {:?}", id);
                },
                MeetingActions::UpdateMemo(memo) => {
                    log::info!("update memo {:?}", memo);
                },
                MeetingActions::LootLeader => {
                    log::info!("Loot leader");
                },
            }
        });
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct MeetingProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MeetingProvider)]
pub fn meeting_provider(props: &MeetingProviderProps) -> Html {
    let state = use_state(|| MeetingState {
        id: String::from(""),
        leader_id: None,
        members: Vec::new(),
        memo: String::from(""),
        error_msg: None,
    });

    let model = MeetingContext::new(state);
    html! {
        <ContextProvider<MeetingContext> context={model}>
            {props.children.clone()}
        </ContextProvider<MeetingContext>>
    }
}
