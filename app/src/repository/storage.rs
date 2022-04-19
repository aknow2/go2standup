use web_sys::{ window };

pub fn get_meeting_id() -> Option<String> {
  let meeting_id_key = "meeting_id";
  let storage = window().unwrap().local_storage().unwrap().unwrap();
  return storage.get_item(meeting_id_key).unwrap();
}

pub fn set_meeting_id(mid: &str) {
  let meeting_id_key = "meeting_id";
  let storage = window().unwrap().local_storage().unwrap().unwrap();
  storage.set_item(meeting_id_key, mid).unwrap();
}
