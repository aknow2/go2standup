use redis::Commands;
use async_graphql::*;
use futures::lock::Mutex;
use std::result::Result;
use serde_json::*;
use serde:: { Serialize, Deserialize };

pub type MeetingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject, Serialize, Deserialize)]
pub struct Meeting {
    id: ID,
    leader_id: Option<String>,
}

pub type Storage = Mutex<redis::Client>;

pub struct QueryRoot;

pub type RetriveMeetingResult = Result<Meeting, String>;
#[Object]
impl QueryRoot {
    async fn meeting(&self, ctx: &Context<'_>) -> RetriveMeetingResult {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        let mut conn = storage
            .get_connection()
            .expect( "Failed to connect storage");
        let id = uuid::Uuid::new_v4().to_string();
        let _: () = conn.set("test", "test_data").unwrap();
        let meeting = Meeting {
            id: ID(id),
            leader_id: None,
        };

        let data: String = conn.get("test").unwrap();
        print!("This data {:?}", data);
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
            };
        let _: () = conn.set(id, serde_json::from_value(meeting)).unwrap();
        Ok(meeting)
    }
}
