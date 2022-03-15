use crate::data;

pub struct Repository {
    storage: web_sys::Storage,
}

impl Repository {
    pub fn new() -> Option<Repository> {
        let window = web_sys::window().unwrap();

        if let Ok(Some(storage)) =  window.local_storage(){
            Some(Repository {
                storage,
            })
        } else {
            None
        }
    }

    pub fn save_members(&self, members: &data::member::Members) {
        let text = serde_json::to_string(members).unwrap();
        log::info!("save_members: {:?}", text);
        self.storage.set_item("members", &text).unwrap();
    }

    pub fn fetch_members(&self) -> data::member::Members {
        let text = self.storage.get_item("members").unwrap().unwrap_or_else(|| String::from("[]"));
        let result = serde_json::from_str::<data::member::Members>(&text).unwrap_or_else(|_| Vec::new());
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