use crate::data;
use graphql_client::{GraphQLQuery};
use wasm_bindgen::{JsCast};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, RequestInit, Response as Res, window, RequestMode };
use data::meeting::{Meeting, GQLResponse, MeetingHolder, CreateMeetingHolder };


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

pub struct Repository {
    storage: web_sys::Storage,
}

async fn fetch_meeting(id: String) -> Meeting {
    let window = window().unwrap();
    let variables = fetch_meeting::Variables {
        id: id,
    };

    let build_query = FetchMeeting::build_query(variables);
    let query = serde_json::json!(build_query);
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);

    let url = "http://localhost:7070";
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Res = resp_value.dyn_into().unwrap();
    let result_json = resp.json().unwrap();
    let json = JsFuture::from(result_json).await.unwrap();
    let response: GQLResponse<MeetingHolder> = json.into_serde().unwrap();

    return response.data.unwrap().meeting;
}

async fn create_meeting() -> Meeting {
    let window = window().unwrap();
    let variables = create_meeting::Variables {};

    let build_query = CreateMeeting::build_query(variables);
    let query = serde_json::json!(build_query);
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);

    let url = "http://localhost:7070";
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Res = resp_value.dyn_into().unwrap();
    let result_json = resp.json().unwrap();
    let json = JsFuture::from(result_json).await.unwrap();
    let response: GQLResponse<CreateMeetingHolder> = json.into_serde().unwrap();

    return response.data.unwrap().create_meeting;
}

impl Repository {
    pub async fn new(id: Option<String>) -> Option<Repository> {
        let window = window().unwrap();

        let meeting = match id {
            Some(id) => fetch_meeting(id).await,
            None => create_meeting().await,
        };

        log::info!("meeting {:?}", meeting);

        if let Ok(Some(storage)) =  window.local_storage(){
            Some(Repository {
                storage,
            })
        } else {
            None
        }
    }

    pub fn save_members(&self, members: &data::meeting::Members) {
        let text = serde_json::to_string(members).unwrap();
        log::info!("save_members: {:?}", text);
        self.storage.set_item("members", &text).unwrap();
    }

    pub fn fetch_members(&self) -> data::meeting::Members {
        let text = self.storage.get_item("members").unwrap().unwrap_or_else(|| String::from("[]"));
        let result = serde_json::from_str::<data::meeting::Members>(&text).unwrap_or_else(|_| Vec::new());
        log::info!("fetch_members: {:?}", result);
        result
    }

    pub fn save_memo(&self, memo: &str) {
        log::info!("save_memo: {:?}", memo);
        self.storage.set_item("memo", memo).unwrap();
    }

    pub fn fetch_memo(&self) -> String {
        let memo = self.storage.get_item("memo").unwrap().unwrap_or_else(|| String::from(""));
        log::info!("fetch_memo: {:?}", memo);
        memo
    }
}