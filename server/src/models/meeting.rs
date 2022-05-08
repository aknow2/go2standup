use redis::{Commands, RedisError, ErrorKind };
use rand::prelude::SliceRandom;
use async_graphql::*;
use futures::{lock::Mutex, Stream };
use futures_util::StreamExt as _;
use std::{result::Result};
use serde:: { Serialize, Deserialize };

pub type MeetingSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReactionType {
    NONE,
    THUMBSUP,
    THUMBSDOWN,
    SMILE,
    CLAP,
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
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

async fn save_meeting(ctx: &Context<'_>, id: String,mut cb: impl FnMut(Meeting) -> CreateMeetingResult) -> CreateMeetingResult {
    let storage = ctx.data_unchecked::<Storage>().lock().await;

    let mut conn = storage
        .get_connection()
        .or_else(|_| Err(String::from("Failed to connect storage")))?;
    let cloned_id = id.clone();
    let result: Result<Meeting, RedisError> = redis::transaction(&mut conn, &[id], move |con, pipe| {
        let id = &cloned_id;
        let data: String = con
            .get(&id)?;
        let meeting: Meeting = serde_json::from_str(&data).or_else(|_| {
                Err(
                    RedisError::from((ErrorKind::TypeError, "Meeting object is broken"))
                )
            })?;
        let new_meeting = cb(meeting).or_else(|_| {
                Err(
                    RedisError::from((ErrorKind::TypeError, "Failed to update meeting"))
                )
            })?;
        let json_str: String = serde_json::to_string(&new_meeting)
            .or_else(|_| {
                Err(
                    RedisError::from((ErrorKind::TypeError, "Failed to covert json"))
                )
            })?;
        pipe.set(&id, json_str.clone()).ignore().query(con)?;
        con.publish::<String, String, i32>(id.to_string(), json_str).unwrap();
        Ok(Some(new_meeting))
    });
    match result {
        Ok(meeting) =>  Ok(meeting),
        Err(er) => {
            let err_msg = match er.detail() {
                Some(detail) => String::from(detail),
                None => String::from("Unexpected redis error")
            };
            Err(err_msg)
        }
    }


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
                name: name.clone(),
                reaction: ReactionType::NONE,
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
            let _ = std::mem::replace(
                &mut meeting.members[index],
                Member {
                        id: member.id.clone(),
                        name: member.name.clone(),
                        reaction: member.reaction
                    });
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
    async fn shuffle_members(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
    ) -> CreateMeetingResult {
        let save_memo = move |m: Meeting| {
            let mut meeting = m.clone();
            let mut rng = rand::thread_rng();
            let mut member_list = meeting.members.to_vec();
            member_list.shuffle(&mut rng);
            meeting.members = member_list;
            Ok(meeting)
        };
        save_meeting(ctx, id, save_memo).await
    }
    async fn new_leader(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
    ) -> CreateMeetingResult {
        let save_memo = move |m: Meeting| {
            let mut meeting = m.clone();
            let mut rng = rand::thread_rng();
            let mut member_list = meeting.members.to_vec();
            member_list.shuffle(&mut rng);
            
            let maybe_leader = member_list.get(0);
            if let Some(leader) = maybe_leader {
                meeting.leader_id = Some(leader.id.to_string());
            }
            Ok(meeting)
        };
        save_meeting(ctx, id, save_memo).await
    }
    async fn update_memo(&self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the meeting")] id: String,
        #[graphql(desc = "memo")] memo: String,
    ) -> CreateMeetingResult {
        let save_memo = move |m: Meeting| {
            let mut meeting = m.clone();
            meeting.memo = memo.clone();
            Ok(meeting)
        };
        save_meeting(ctx, id, save_memo).await
    }
}


pub struct SubscriptionRoot;
#[Subscription]
impl SubscriptionRoot {
    async fn meeting(&self, ctx: &Context<'_>, #[graphql(desc = "Id of meeting")] id: String) -> impl Stream<Item = Result<Meeting, String>> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;

        let client = storage.clone();
        println!("start subscribe {:?}", &id);
        async_stream::stream! {
            let mut pubsub_conn = client.get_async_connection().await.unwrap().into_pubsub();
            pubsub_conn.subscribe(&id).await.unwrap();
            let mut pubsub_stream = pubsub_conn.on_message();
            while let Some(next) = pubsub_stream.next().await {
                let payload : String = next.get_payload().unwrap();
                println!("channel meeting '{:?}' ", &id);
                let meeting: Meeting = serde_json::from_str(&payload).unwrap();
                yield Ok(meeting);
            }
        }
    }
}
