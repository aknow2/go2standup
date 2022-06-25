use crate::data;
use crate::data::meeting::{ErrorMsg, AddMemberHolder, Member, RemoveMemberHolder, UpdateMemberHolder, UpdateMemoHolder, ReactionType, ShuffleMembersHolder, NewLeaderHolder, NotificationEventHolder, NotificationEvent};
use crate::repository::gql_protocol::{connection_init_msg, subscribe_msg, RecivedMsg};
use graphql_client::{GraphQLQuery};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, RequestInit, Response as Res, window, RequestMode, WebSocket, MessageEvent };
use data::meeting::{Meeting, GQLResponse, MeetingHolder, CreateMeetingHolder };

async fn post(query: serde_json::Value, url:&str) -> JsValue {
    let window = window().unwrap();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts).unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Res = resp_value.dyn_into().unwrap();
    let result_json = resp.json().unwrap();
    JsFuture::from(result_json).await.unwrap()
}

pub type MeetingResult = Result<Meeting, Vec<ErrorMsg>>;
pub type NotificationResult = Result<NotificationEvent, Vec<ErrorMsg>>;
type ParseResCB<T, R> = fn(T) -> R;
fn parse_response<T, R>(response: GQLResponse<T>, get_value: ParseResCB<T, R>)-> Result<R, Vec<ErrorMsg>> {
    if let Some(data) = response.data {
        let meeting = get_value(data);
        return Ok(meeting);
    }
    if let Some(data) = response.errors {
        return Err(data);
    }
    if let Some(data) = response.error {
        return Err(Vec::from([
            ErrorMsg {
                message: data,
            },
        ]));
    }
    Err(Vec::from(
        [
            ErrorMsg {
                message: String::from("Unexpected error occured"),
            }
        ]
    ))
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/fetch_meeting.graphql",
    response_derives = "Debug"
)]
struct FetchMeeting;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/create_meeting.graphql",
    response_derives = "Debug"
)]
struct CreateMeeting;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/add_member.graphql",
    response_derives = "Debug"
)]
struct AddMember;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/shuffle_members.graphql",
    response_derives = "Debug"
)]
struct ShuffleMembers;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/new_leader.graphql",
    response_derives = "Debug"
)]
struct NewLeader;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/remove_member.graphql",
    response_derives = "Debug"
)]
struct RemoveMember;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/update_member.graphql",
    response_derives = "Debug"
)]
struct UpdateMember;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/update_memo.graphql",
    response_derives = "Debug"
)]
struct UpdateMemo;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/subscription_meeting.graphql",
    response_derives = "Debug"
)]
struct SubscribeMeeting;


#[derive(Debug, PartialEq, Clone)]
pub struct API {
    origin: String,
    secure: bool,
}

impl API {
    pub fn new() -> API {
        let origin = option_env!("API_ORIGIN").expect("API_ORIGIN is undefined");
        let secure_flag = option_env!("SECURE");
        Self {
            origin: origin.to_string(),
            secure: match secure_flag {
                Some(_) => true,
                None => false,
            },
        }
    }

    fn url(&self) -> String {
        if self.secure {
            return "https://".to_owned() + &self.origin
        }
        "http://".to_owned() + &self.origin
    }

    fn ws(&self) -> String {
        if self.secure {
            return "wss://".to_owned() + &self.origin + "/ws"
        }
        "ws://".to_owned() + &self.origin + "/ws"
    }

    pub async fn update_memo(&self, id: String, memo: String) -> MeetingResult {
        let variables = update_memo::Variables {
            id,
            memo,
        };
        let build_query = UpdateMemo::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<UpdateMemoHolder> = json.into_serde().unwrap();
        parse_response(
            response, |
            d|
            d.update_memo
        )
    }
    pub async fn fetch_meeting(&self, id: String) -> MeetingResult {
        let variables = fetch_meeting::Variables {
            id: id,
        };

        let build_query = FetchMeeting::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<MeetingHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.meeting )
    }

    pub async fn update_member(&self, id: String, member: Member) -> MeetingResult {
        let reaction = match member.reaction {
            ReactionType::ZERO => update_member::ReactionType::ZERO,
            ReactionType::I => update_member::ReactionType::I,
            ReactionType::II => update_member::ReactionType::II,
            ReactionType::III => update_member::ReactionType::III,
            ReactionType::IV => update_member::ReactionType::IV,
            ReactionType::V => update_member::ReactionType::V,
            ReactionType::VI => update_member::ReactionType::VI,
            ReactionType::VII => update_member::ReactionType::VII,
            ReactionType::VIII => update_member::ReactionType::VIII,
            ReactionType::IX => update_member::ReactionType::IX,
            ReactionType::X => update_member::ReactionType::X,
            ReactionType::THUMBSUP => update_member::ReactionType::THUMBSUP,
            ReactionType::THUMBSDOWN => update_member::ReactionType::THUMBSDOWN,
            ReactionType::SMILE => update_member::ReactionType::SMILE,
            ReactionType::CLAP => update_member::ReactionType::CLAP,
            ReactionType::NONE => update_member::ReactionType::NONE,
        };

        let variables = update_member::Variables {
            id,
            member_id: member.id,
            reaction,
            name: member.name,
        };
        let build_query = UpdateMember::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<UpdateMemberHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.update_member)
    }

    pub async fn remove_member(&self, id: String, member_id: String) -> MeetingResult {
        let variables = remove_member::Variables {
            id,
            member_id,
        };
        let build_query = RemoveMember::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<RemoveMemberHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.remove_member)
    }

    pub async fn create_meeting(&self) -> MeetingResult {
        let variables = create_meeting::Variables {};
        let build_query = CreateMeeting::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<CreateMeetingHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.create_meeting)
    }

    pub async fn add_member(&self, id: String, name: String) -> MeetingResult {
        let variables = add_member::Variables {
            id,
            name,
        };
        let build_query = AddMember::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<AddMemberHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.add_member)
    }
    pub async fn shuffle_members(&self, id: String) -> MeetingResult {
        let variables = shuffle_members::Variables {
            id,
        };
        let build_query = ShuffleMembers::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<ShuffleMembersHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.shuffle_members)
    }

    pub async fn new_leader(&self, id: String) -> MeetingResult {
        let variables = new_leader::Variables {
            id,
        };
        let build_query = NewLeader::build_query(variables);
        let query = serde_json::json!(build_query);
        let json = post(query, &self.url()).await;
        let response: GQLResponse<NewLeaderHolder> = json.into_serde().unwrap();
        parse_response(response, |d| d.new_leader)
    }

    pub fn subscribe_meeting(&self, id: String, mut cb: Box<dyn FnMut(NotificationResult)>)  {

        let ws = WebSocket::new_with_str(&self.ws(), "graphql-ws").unwrap();
        {
            let onmessage_callback = Closure::wrap(
            Box::new(move |e: MessageEvent| {
                    log::info!("Received data {:?}", e.data());
                    let msg: RecivedMsg<NotificationEventHolder> = serde_json::from_str(&e.data().as_string().unwrap()).unwrap();
                    if let Some(payload) = msg.payload {
                        cb(parse_response(payload, |holder| holder.notification));
                    }
                }) as Box<dyn FnMut(MessageEvent)>
            );
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
        }

        {
            let cloned_ws = ws.clone();
            let onopen_callback = Closure::wrap(Box::new(move |_| {
                match cloned_ws.send_with_str(&connection_init_msg(None).to_string()) {
                    Ok(_) => log::info!("message successfully sent connection msg"),
                    Err(err) => log::info!("error sending message: {:?}", err),
                }
                let variables = subscribe_meeting::Variables {
                    id: id.to_string(),
                };
                let build_query = SubscribeMeeting::build_query(variables);
                let query = serde_json::json!(build_query);
                log::info!("socket opened");
                let ws_id = uuid::Uuid::new_v4();
                match cloned_ws.send_with_str(&subscribe_msg(&ws_id.to_string(), query).to_string()) {
                    Ok(_) => log::info!("message successfully sent subscribe"),
                    Err(err) => log::info!("error sending message: {:?}", err),
                }
            }) as Box<dyn FnMut(JsValue)>);
            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();
        }

        let onerror_callback = Closure::wrap(Box::new(move |er| {
            log::error!("socket error {:?}", er);
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    }
}
