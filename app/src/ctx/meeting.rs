use std::{rc::Rc};

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{
    data::meeting:: { Member, ErrorMsg, ReactionType, NotificationEvent },
    repository::{storage::{get_meeting_id, set_meeting_id},
    api::{MeetingResult, API, NotificationResult
}}};
pub enum MeetingActions {
    StartMeeting(Option<String>),
    UpdateMember(Member),
    UpdateMemo(String),
    AddMember(String),
    RemoveMember(String),
    NewLeader,
    ShuffleMembers,
}

async fn start_meeting(query_id: Option<String>, current: &MeetingState, api: &API) -> MeetingState {

    let meeting_id = match query_id {
        Some(qid) =>  Some(qid),
        None =>  get_meeting_id(),
    };

    let result: MeetingResult = match meeting_id {
        Some(mid) => api.fetch_meeting(mid).await,
        None => api.create_meeting().await,
    };

    match result {
        Ok(meeting) => {
            set_meeting_id(&meeting.id);
            MeetingState {
                id: Some(meeting.id),
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
    pub id: Option<String>,
    pub leader_id: Option<String>,
    pub members: Vec<Member>,
    pub memo: String,
    pub error_msgs: Option<Vec<ErrorMsg>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct MeetingContext {
   pub state: UseStateHandle<MeetingState>,
   api: Rc<API>,
}

pub enum MeetingStatus {
    Initializing,
    Ready,
}

impl MeetingContext {
    fn new(state: UseStateHandle<MeetingState>, api: Rc<API>) -> MeetingContext {
        MeetingContext {
            state,
            api,
        }
    }

    pub fn meeting_status(&self) -> MeetingStatus {
        match self.state.id {
            Some(_) => MeetingStatus::Ready,
            None => MeetingStatus::Initializing,
        }
    }

    fn received_meeting_result(&self, result: MeetingResult) {
        let state = self.state.clone();
        match result {
            Ok(meeting) => {
                log::info!("{:?}", meeting);
                state.set(MeetingState {
                    id: Some(meeting.id),
                    members: meeting.members,
                    memo: meeting.memo,
                    leader_id: meeting.leader_id.clone(),
                    error_msgs: None,
                })
            },
            Err(msg) => {
                log::error!("{:?}", msg);
                state.set(MeetingState {
                    error_msgs: Some(msg),
                    ..(*state).clone()
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
                    let new_state = start_meeting(id, &state, &my.api).await;
                    log::info!("start meeting {:?}", new_state);
                    let api = Rc::clone(&my.api);
                    let func = Box::new(move |result: NotificationResult | {
                        log::info!("subscribe {:?}", result);
                        match result {
                            Ok(result) => match result {
                                NotificationEvent::Meeting(m) => my.received_meeting_result(Ok(m)),
                                NotificationEvent::Reaction(_) => (),
                            },
                            Err(e) => my.received_meeting_result(Err(e)),
                        };
                    });
                    if let Some(id) = &new_state.id {
                        api.subscribe_meeting(id.to_string(), func);
                        state.set(new_state);
                    }
                },
                MeetingActions::AddMember(name) => {
                    if let Some(id) = &state.id {
                        let result = my.api.add_member(id.clone(), name).await;
                        my.received_meeting_result(result);
                    }
                },
                MeetingActions::RemoveMember(member_id) => {
                    if let Some(id) = &state.id {
                        log::info!("remove member {:?}", member_id);
                        let result = my.api.remove_member(
                            id.clone(),
                            member_id,
                        ).await;
                        my.received_meeting_result(result);
                    }
                },
                MeetingActions::UpdateMemo(memo) => {
                    if let Some(id) = &state.id {
                        log::info!("update memo {:?}", memo);
                        let result = my.api.update_memo(
                            id.clone(),
                            memo,
                        ).await;
                        my.received_meeting_result(result);
                    }
                },
                MeetingActions::ShuffleMembers => {
                    if let Some(id) = &state.id {
                        let result = my.api.shuffle_members(id.clone()).await;
                        my.received_meeting_result(result);
                        log::info!("Shffule members");
                    }
                },
                MeetingActions::NewLeader => {
                    if let Some(id) = &state.id {
                        log::info!("New leader");
                        let result = my.api.new_leader(id.clone()).await;
                        my.received_meeting_result(result);
                    }
                },
                MeetingActions::UpdateMember(member) => {
                    if let Some(id) = &state.id {
                        log::info!("New leader");
                        let result = my.api.update_member(id.clone(), member).await;
                        my.received_meeting_result(result);
                    }
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

#[derive(Debug, PartialEq, Clone)]
struct APIContainer {
    api: Rc<API>
}

#[function_component(MeetingProvider)]
pub fn meeting_provider(props: &MeetingProviderProps) -> Html {
    let state = use_state(|| MeetingState {
        id: None,
        leader_id: None,
        members: Vec::new(),
        memo: String::from(""),
        error_msgs: None,
    });
    let api_container = use_state(|| APIContainer {
        api: Rc::from(API::new()),
    });
    let model = MeetingContext::new(state, Rc::clone(&api_container.api));
    {
        let ctx = model.clone();
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
        <ContextProvider<MeetingContext> context={model}>
            {props.children.clone()}
        </ContextProvider<MeetingContext>>
    }
}
