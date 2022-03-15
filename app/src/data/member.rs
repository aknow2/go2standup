use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Member {
    pub id: usize,
    pub name: String,
}

pub type Members = Vec<Member>;
