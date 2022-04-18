use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{data::meeting:: { Member, ErrorMsg }, repository::{storage::{get_meeting_id, set_meeting_id}, api::{fetch_meeting, create_meeting, add_member, MeetingResult, remove_member, update_memo, subscribe_meeting, shuffle_members, new_leader}}};
pub enum MeetingActions {
    StartMeeting(Option<String>),
    UpdateMemo(String),
    AddMember(String),
    RemoveMember(String),
    NewLeader,
    ShuffleMembers,
}

async fn start_meeting(query_id: Option<String>, current: &MeetingState) -> MeetingState {

    let meeting_id = match query_id {
        Some(qid) =>  Some(qid),
        None =>  get_meeting_id(),
    };

    let result: MeetingResult = match meeting_id {
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
                error_msgs: Some(msg),
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
    pub error_msgs: Option<Vec<ErrorMsg>>
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

    fn received_meeting_result(&self, result: MeetingResult) {
        let state = self.state.clone();
        match result {
            Ok(meeting) => {
                log::info!("{:?}", meeting);
                state.set(MeetingState {
                    id: meeting.id,
                    members: meeting.members,
                    memo: meeting.memo,
                    leader_id: meeting.leader_id.clone(),
                    error_msgs: None,
                })
            },
            Err(msg) => {
                log::error!("{:?}", msg);
                state.set(MeetingState {
                    id: state.id.to_string(),
                    members: state.members.to_vec(),
                    memo: state.memo.to_string(),
                    leader_id: state.leader_id.clone(),
                    error_msgs: Some(msg),
                })
            },
        }
    }

    pub fn dispatch(&self, action: MeetingActions) {
        let state = self.state.clone();
        let my = self.clone();
        spawn_local( async move  {
            match action {
                MeetingActions::StartMeeting(id) => {
                    let new_state = start_meeting(id, &state).await;
                    log::info!("start meeting {:?}", new_state);
                    let func = Box::new(move |result: MeetingResult | {
                        log::info!("subscribe {:?}", result);
                        my.received_meeting_result(result);
                    });
                    subscribe_meeting(new_state.id.to_string(), func);
                    state.set(new_state);
                },
                MeetingActions::AddMember(name) => {
                    let result = add_member(state.id.clone(), name).await;
                    my.received_meeting_result(result);
                },
                MeetingActions::RemoveMember(member_id) => {
                    log::info!("remove member {:?}", member_id);
                    let result = remove_member(
                        state.id.clone(),
                        member_id,
                    ).await;
                    my.received_meeting_result(result);
                },
                MeetingActions::UpdateMemo(memo) => {
                    log::info!("update memo {:?}", memo);
                    let result = update_memo(
                        state.id.clone(),
                        memo,
                    ).await;
                    my.received_meeting_result(result);
                },
                MeetingActions::ShuffleMembers => {
                    let result = shuffle_members(state.id.clone()).await;
                    my.received_meeting_result(result);
                    log::info!("Shffule members");
                },
                MeetingActions::NewLeader => {
                    log::info!("New leader");
                    let result = new_leader(state.id.clone()).await;
                    my.received_meeting_result(result);
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
        error_msgs: None,
    });
    
    let model = MeetingContext::new(state);
    html! {
        <ContextProvider<MeetingContext> context={model}>
            {props.children.clone()}
        </ContextProvider<MeetingContext>>
    }
}
