use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
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

#[derive(Deserialize, Serialize)]
pub struct DataPayload<T> {
  pub data: T,
}

#[derive(Deserialize, Serialize)]
pub struct RecivedMsg<T> {
  #[serde(rename = "type")]
  pub msg_type: String,
  pub id: String,
  pub payload: DataPayload<T>,
}
