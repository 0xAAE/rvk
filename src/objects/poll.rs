use super::*;

/// <https://vk.com/dev/objects/poll>
#[derive(Deserialize, Clone, Debug)]
pub struct Poll {
    pub id: Integer,
    pub owner_id: Integer,
    pub created: Integer,
    pub question: String,
    #[serde(default)]
    pub votes: Integer,
    pub answers: Vec<Answer>,
    #[serde(default)]
    pub anonymous: Boolean,
    #[serde(default)]
    pub multiple: Boolean,
    pub answer_ids: Option<Vec<Integer>>,
    pub end_date: Integer,
    #[serde(default)]
    pub closed: Boolean,
    #[serde(default)]
    pub is_board: Boolean,
    #[serde(default)]
    pub can_edit: Boolean,
    #[serde(default)]
    pub can_vote: Boolean,
    #[serde(default)]
    pub can_report: Boolean,
    #[serde(default)]
    pub can_share: Boolean,
    pub author_id: Option<Integer>, // optional at least in newsfeed
    pub photo: Option<photo::Photo>,
    pub background: Option<Background>,
    pub friends: Option<Vec<PollFriend>>, // optional at least in newsfeed
}

#[derive(Deserialize, Clone, Debug)]
pub struct Answer {
    pub id: Integer,
    pub text: String,
    pub votes: Integer,
    pub rate: Number,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Background {
    pub id: Integer,

    #[serde(rename = "type")]
    pub type_: String,

    pub angle: Option<Integer>,
    pub color: String,
    pub width: Option<Integer>,
    pub height: Option<Integer>,
    pub images: Option<photo::Size>,
    pub points: Vec<Option<GradientPoint>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GradientPoint {
    pub position: Number,
    pub color: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PollFriend {
    pub id: Integer,
}
