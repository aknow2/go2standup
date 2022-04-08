use serde:: { Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReactionType {
    NONE,
    Thumbup,
    Thumbdown,
    I,
    II,
    III,
    V,
    VIII,
    XIII,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub reaction: ReactionType,
}

pub type Members = Vec<Member>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InputMember {
    pub id: String,
    pub name: String,
    pub reaction: ReactionType,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    pub id: String,
    pub leader_id: Option<String>,
    pub members: Vec<Member>,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MeetingHolder {
    pub meeting: Meeting,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeetingHolder {
    pub create_meeting: Meeting,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct  ErrorMsg {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GQLResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub errors: Option<ErrorMsg>
}