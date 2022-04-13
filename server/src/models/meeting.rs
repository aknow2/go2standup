use redis::{Commands };
use async_graphql::*;
use futures::{lock::Mutex, Stream };
use futures_util::StreamExt as _;
use std::{result::Result};
use serde:: { Serialize, Deserialize };

pub type MeetingSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
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

#[derive(InputObject)]
pub struct InputMember {
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
            .or_else(|_| Err(String::from("Failed to connect storage")))?;
        let data: String = conn.get(id)
            .or_else(|_| Err(String::from("Invalid meeting id")))?;
        print!("This data {:?}", data);
        let meeting: Meeting = serde_json::from_str(&data)
            .or_else(|_| Err(String::from("failed to convert Meeting")))?;
        Ok(meeting)
    }
}

async fn save_meeting(ctx: &Context<'_>, id: String, cb: impl FnOnce(Meeting) -> CreateMeetingResult) -> CreateMeetingResult {
    let storage = ctx.data_unchecked::<Storage>().lock().await;
    let mut conn = storage
        .get_connection()
        .or_else(|_| Err(String::from("Failed to connect storage")))?;
    let data: String = conn
        .get(&id)
        .or_else(|_| Err(String::from("Invalid meeting id")))?;
    let meeting: Meeting = serde_json::from_str(&data)
        .or_else(|_| Err(String::from("failed to convert Meeting")))?;
    let new_meeting = cb(meeting)?;
    let json_str: String = serde_json::to_string(&new_meeting)
        .or_else(|_| Err(String::from("failed to conver to json")))?;
    let _: () = conn.set(&id, json_str.clone())
        .or_else(|_| Err(String::from("failed to save meeting")))?;
    conn.publish::<String, String, i32>(id, json_str.clone()).unwrap();
    Ok(new_meeting)
}

pub type CreateMeetingResult = Result<Meeting, String>;
pub struct MutationRoot;

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
    async fn add_member(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
        #[graphql(desc = "name of member")] name: String,
    ) -> CreateMeetingResult {
        save_meeting(ctx, id, move |m| {
            let mut meeting = m.clone();
            let member_id = uuid::Uuid::new_v4().to_string();
            let member = Member {
                id: ID(member_id),
                name: name,
                reaction: ReactionType::None,
            };        
            meeting.members.push(member);
            Ok(meeting)
        }).await
    }
    async fn update_member(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
        #[graphql(desc = "struct of member")] member: InputMember,
    ) -> CreateMeetingResult {
        save_meeting(ctx, id, move |m| {
            let member_id = member.id.to_string();
            let mut meeting = m.clone();
            let index_result = meeting
                .members
                .clone()
                .iter()
                .position(|m| m.id.to_string() == member_id);
            let index = match index_result {
                Some(i) => i,
                None => return Err(String::from("Invalid member id"))
            };
            let _ = std::mem::replace(&mut meeting.members[index], Member { id: member.id, name: member.name, reaction: member.reaction });
            Ok(meeting)
        }).await
    }
    async fn remove_member(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
        #[graphql(desc = "id of member")] member_id: String,
    ) -> CreateMeetingResult {
        save_meeting(ctx, id, move |m| {
            let mut meeting = m.clone();
            meeting.members = meeting
                .members
                .iter()
                .filter(|m| m.id.to_string() != member_id)
                .cloned()
                .collect();
            Ok(meeting)
        }).await
    }
    async fn update_memo(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
        #[graphql(desc = "memo")] memo: String,
    ) -> CreateMeetingResult {
        let save_memo = move |m: Meeting| {
            let mut meeting = m.clone();
            meeting.memo = memo;
            Ok(meeting)
        };
        save_meeting(ctx, id, save_memo).await
    }
}


pub struct SubscriptionRoot;
#[Subscription]
impl SubscriptionRoot {
    async fn meeting(&self, #[graphql(desc = "Id of meeting")] id: String) -> impl Stream<Item = Result<Meeting, String>> {
        let client = redis::Client::open("redis://redis/").expect("failed to open redis");
        async_stream::stream! {
            let mut pubsub_conn = client.get_async_connection().await.unwrap().into_pubsub();
            pubsub_conn.subscribe(&id).await.unwrap();
            let mut pubsub_stream = pubsub_conn.on_message();
            while let Some(next) = pubsub_stream.next().await {
                let payload : String = next.get_payload().unwrap();
                println!("channel meeting '{}' ", &id);
                let meeting: Meeting = serde_json::from_str(&payload).unwrap();
                yield Ok(meeting);
            }
        }
    }
}
