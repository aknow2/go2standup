use crate::data;
use crate::data::meeting::{ErrorMsg};
use graphql_client::{GraphQLQuery};
use wasm_bindgen::{JsCast};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, RequestInit, Response as Res, window, RequestMode };
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

type MeetingResult = Result<Meeting, ErrorMsg>;
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
pub async fn fetch_meeting(id: String) -> Result<Meeting, ErrorMsg> {
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
