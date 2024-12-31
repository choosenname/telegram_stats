use crate::utils::deserialize_datetime::deserialize_datetime;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub r#type: String,
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub r#type: String,
    pub action: Option<String>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub date: DateTime<chrono::Utc>,
    pub date_unixtime: String,
    pub from: Option<String>,
    pub from_id: Option<String>,
    pub edited: Option<String>,
    pub edited_unixtime: Option<String>,
    pub reply_to_message_id: Option<i64>,
    pub text: MessageText,
    pub text_entities: Vec<MessageEntity>,
    pub members: Option<Vec<String>>,
    pub actor: Option<String>,
    pub actor_id: Option<String>,
    pub photo: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file: Option<String>,
    pub thumbnail: Option<String>,
    pub self_destruct_period_seconds: Option<i32>,
    pub title: Option<String>,
    pub inviter: Option<String>,
    pub message_id: Option<i64>,
    pub game_message_id: Option<i64>,
    pub score: Option<i32>,
    pub amount: Option<i32>,
    pub currency: Option<String>,
    pub invoice_message_id: Option<i64>,
    pub recurring: Option<String>,
    pub duration_seconds: Option<i32>,
    pub discard_reason: Option<String>,
    pub information_text: Option<String>,
    pub reason_app_id: Option<i32>,
    pub reason_app_name: Option<String>,
    pub reason_domain: Option<String>,
    pub values: Option<Vec<String>>,
    pub to_id: Option<i32>,
    pub to: Option<String>,
    pub distance: Option<i32>,
    pub period: Option<i32>,
    pub schedule_date: Option<i32>,
    pub emoticon: Option<String>,
    pub cost: Option<i32>,
    pub months: Option<i32>,
    pub new_title: Option<String>,
    pub new_icon_emoji_id: Option<String>,
    pub button_id: Option<i32>,
    pub peer_id: Option<i32>,
    pub author: Option<String>,
    pub forwarded_from: Option<String>,
    pub saved_from: Option<String>,
    pub via_bot: Option<String>,
    pub media_type: Option<String>,
    pub performer: Option<String>,
    pub mime_type: Option<String>,
    pub contact_information: Option<Contact>,
    pub contact_vcard: Option<String>,
    pub location_information: Option<Location>,
    pub live_location_period_seconds: Option<i32>,
    pub place_name: Option<String>,
    pub address: Option<String>,
    pub game_title: Option<String>,
    pub game_description: Option<String>,
    pub game_link: Option<String>,
    pub invoice_information: Option<Invoice>,
    pub poll: Option<Poll>,
    pub gift_code: Option<String>,
    pub boost_peer_id: Option<i32>,
    pub unclaimed: Option<bool>,
    pub via_giveaway: Option<bool>,
    pub giveaway_information: Option<Giveaway>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageText {
    Plain(String),
    Entities(Vec<TextEntity>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextEntity {
    Text(String),
    Entity(MessageEntity),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    pub r#type: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone_number: String,
    pub date: Option<String>,
    pub date_unixtime: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub amount: i32,
    pub currency: String,
    pub receipt_message_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Poll {
    pub question: String,
    pub closed: bool,
    pub total_voters: i32,
    pub answers: Vec<PollAnswer>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollAnswer {
    pub text: String,
    pub voters: i32,
    pub chosen: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Giveaway {
    pub quantity: i32,
    pub months: i32,
    pub until_date: String,
    pub channels: Vec<i32>,
}
