use crate::data;
use crate::data::meeting::{ErrorMsg, AddMemberHolder, Member, RemoveMemberHolder, UpdateMemberHolder, UpdateMemoHolder, ReactionType};
use crate::repository::gql_protocol::{connection_init_msg, subscribe_msg};
use graphql_client::{GraphQLQuery};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, RequestInit, Response as Res, window, RequestMode, WebSocket, MessageEvent };
use data::meeting::{Meeting, GQLResponse, MeetingHolder, CreateMeetingHolder };

async fn post(query: serde_json::Value) -> JsValue {
    let window = window().unwrap();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);

    let url = "http://localhost:7070";
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Res = resp_value.dyn_into().unwrap();
    let result_json = resp.json().unwrap();
    JsFuture::from(result_json).await.unwrap()
}

pub type MeetingResult = Result<Meeting, ErrorMsg>;
type ParseResCB<T> = fn(T) -> Meeting;
fn parse_response<T>(response: GQLResponse<T>, get_value: ParseResCB<T>)-> MeetingResult {
    if let Some(data) = response.data {
        let meeting = get_value(data);
        return Ok(meeting);
    }
    if let Some(data) = response.errors {
        return Err(data);
    }
    if let Some(data) = response.error {
        return Err(ErrorMsg {
            message: data,
        });
    }
    Err(ErrorMsg {
        message: String::from("Unexpected error occured"),
    })
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/fetch_meeting.graphql",
    response_derives = "Debug"
)]
struct FetchMeeting;
pub async fn fetch_meeting(id: String) -> MeetingResult {
    let variables = fetch_meeting::Variables {
        id: id,
    };

    let build_query = FetchMeeting::build_query(variables);
    let query = serde_json::json!(build_query);
    let json = post(query).await;
    let response: GQLResponse<MeetingHolder> = json.into_serde().unwrap();
    parse_response(response, |d| d.meeting )
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/create_meeting.graphql",
    response_derives = "Debug"
)]
struct CreateMeeting;
pub async fn create_meeting() -> Result<Meeting, ErrorMsg> {
    let variables = create_meeting::Variables {};
    let build_query = CreateMeeting::build_query(variables);
    let query = serde_json::json!(build_query);
    let json = post(query).await;
    let response: GQLResponse<CreateMeetingHolder> = json.into_serde().unwrap();
    parse_response(response, |d| d.create_meeting)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/add_member.graphql",
    response_derives = "Debug"
)]
struct AddMember;
pub async fn add_member(id: String, name: String) -> MeetingResult {
    let variables = add_member::Variables {
        id,
        name,
    };
    let build_query = AddMember::build_query(variables);
    let query = serde_json::json!(build_query);
    let json = post(query).await;
    let response: GQLResponse<AddMemberHolder> = json.into_serde().unwrap();
    parse_response(response, |d| d.add_member)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/remove_member.graphql",
    response_derives = "Debug"
)]
struct RemoveMember;
pub async fn remove_member(id: String, member_id: String) -> MeetingResult {
    let variables = remove_member::Variables {
        id,
        member_id,
    };
    let build_query = RemoveMember::build_query(variables);
    let query = serde_json::json!(build_query);
    let json = post(query).await;
    let response: GQLResponse<RemoveMemberHolder> = json.into_serde().unwrap();
    parse_response(response, |d| d.remove_member)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/update_member.graphql",
    response_derives = "Debug"
)]
struct UpdateMember;
pub async fn update_member(id: String, member: Member) -> MeetingResult {
    let reaction = match member.reaction {
        ReactionType::I => update_member::ReactionType::I,
        ReactionType::II => update_member::ReactionType::II,
        ReactionType::III => update_member::ReactionType::III,
        ReactionType::V => update_member::ReactionType::V,
        ReactionType::VIII => update_member::ReactionType::VIII,
        ReactionType::XIII => update_member::ReactionType::XIII,
        ReactionType::Thumbup => update_member::ReactionType::Thumbup,
        ReactionType::Thumbdown => update_member::ReactionType::Thumbdown,
        _ => update_member::ReactionType::None,
    };

    let variables = update_member::Variables {
        id,
        member_id: member.id,
        reaction,
        name: member.name,
    };
    let build_query = UpdateMember::build_query(variables);
    let query = serde_json::json!(build_query);
    let json = post(query).await;
    let response: GQLResponse<UpdateMemberHolder> = json.into_serde().unwrap();
    parse_response(response, |d| d.update_member)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/update_memo.graphql",
    response_derives = "Debug"
)]
struct UpdateMemo;
pub async fn update_memo(id: String, memo: String) -> MeetingResult {
    let variables = update_memo::Variables {
        id,
        memo,
    };
    let build_query = UpdateMemo::build_query(variables);
    let query = serde_json::json!(build_query);
    let json = post(query).await;
    let response: GQLResponse<UpdateMemoHolder> = json.into_serde().unwrap();
    parse_response(response, |d| d.update_memo)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/schema.graphql",
    query_path = "src/repository/gql/subscription_meeting.graphql",
    response_derives = "Debug"
)]
struct SubscribeMeeting;
pub fn subscribe_meeting(id: &'static str) {

    let ws = WebSocket::new_with_str("ws://localhost:7070/ws", "graphql-ws").unwrap();
    let onmessage_callback = Closure::wrap(
    Box::new(move |e: MessageEvent| {
                log::info!("{:?}", e.data());
        }) as Box<dyn FnMut(MessageEvent)>
    );
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        let variables = subscribe_meeting::Variables {
            id: id.to_string(),
        };
        let build_query = SubscribeMeeting::build_query(variables);
        let query = serde_json::json!(build_query);
        log::info!("socket opened");
        match cloned_ws.send_with_str(&connection_init_msg(None).to_string()) {
            Ok(_) => log::info!("message successfully sent"),
            Err(err) => log::info!("error sending message: {:?}", err),
        }
        match cloned_ws.send_with_str(&subscribe_msg("2", query).to_string()) {
            Ok(_) => log::info!("message successfully sent"),
            Err(err) => log::info!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |er| {
        log::error!("socket error {:?}", er);
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));


}
