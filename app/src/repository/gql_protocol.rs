use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::data::meeting::GQLResponse;
pub fn connection_init_msg(payload: Option<Value>) -> Value  {
  json!({
    "type": "connection_init",
    "payload": payload,
  })
}

pub fn subscribe_msg(id: &str, payload: Value) -> Value  {
  json!({
    "type": "start",
    "id": id,
    "payload": payload,
  })
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum MsgType {
    connection_ack,
    data,
    error,
}

#[derive(Deserialize, Serialize)]
pub struct RecivedMsg<T> {
  pub r#type: MsgType,
  pub id: Option<String>,
  pub payload: Option<GQLResponse<T>>,
}
