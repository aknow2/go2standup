use redis::Commands;
use async_graphql::*;
use futures::lock::Mutex;
use std::result::Result;
use serde:: { Serialize, Deserialize };

pub type MeetingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReactionType {
    None,
    Thumbup,
    Thumbdown,
    I,
    II,
    III,
    V,
    VIII,
    XIII,
}

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
pub struct Member {
    id: ID,
    name: String,
    reaction: ReactionType,
}

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
pub struct Meeting {
    id: ID,
    leader_id: Option<String>,
    members: Vec<Member>,
    memo: String,
}

pub type Storage = Mutex<redis::Client>;

pub struct QueryRoot;

pub type RetriveMeetingResult = Result<Meeting, String>;
#[Object]
impl QueryRoot {
    async fn meeting(&self, ctx: &Context<'_>, #[graphql(desc = "id of the meeting")] id: String,) -> RetriveMeetingResult {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        let mut conn = storage
            .get_connection()
            .expect( "Failed to connect storage");
        let data: String = conn.get(id).expect("Not found meeting");
        print!("This data {:?}", data);
        let meeting: Meeting = serde_json::from_str(&data).expect("failed to convert Meeting");
        Ok(meeting)
    }
}

pub struct MutationRoot;

pub type CreateMeetingResult = Result<Meeting, String>;
#[Object]
impl MutationRoot {
    async fn create_meeting(&self, ctx: &Context<'_>) -> CreateMeetingResult {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        let mut conn = storage
            .get_connection()
            .expect( "Failed to connect storage");
        let id = uuid::Uuid::new_v4().to_string();
        let meeting = Meeting {
            id: ID(String::from(&id)),
            leader_id: None,
            members: Vec::new(),
            memo: String::from(""),
        };
        let json_str: String = serde_json::to_string(&meeting).expect("failed to conver json");
        let _: () = conn.set(id, json_str).unwrap();
        Ok(meeting)
    }
}
