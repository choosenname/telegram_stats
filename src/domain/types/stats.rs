use crate::domain::types::chat::{Message, MessageText};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct AllStats {
    pub chat_stats: ChatStats,
    pub occurrences: MessagesStats,
    pub longest_conversation: MessagesStats,
    pub calls_stats: CallsStats,
    pub most_used_sticker: MostUsedSticker,
    pub streak: Streak,
}

#[derive(Serialize)]
pub struct ChatStats {
    pub messages_stats: MessagesStats,
    pub additional_messages_stats: AdditionalMessagesStats,
}

#[derive(Serialize)]
pub struct MessagesStats {
    pub first_message: Option<MinimalMessage>,
    pub last_message: Option<MinimalMessage>,
    pub total_messages_count: usize,
    pub owner_messages_count: usize,
    pub member_messages_count: usize,
}

#[derive(Serialize)]
pub struct AdditionalMessagesStats {
    pub total_characters_count: usize,
    pub owner_characters_count: usize,
    pub member_characters_count: usize,
}

#[derive(Serialize)]
pub struct CallsStats {
    pub total_calls_durations_sec: u32,
    pub total_calls_durations_min: u32,
    pub longest_call_durations_min: Option<MinimalMessage>,
}

#[derive(Serialize)]
pub struct MostUsedSticker {
    pub owner_most_used_sticker_count: i32,
    pub owner_most_used_sticker: Option<MinimalMessage>,
    pub member_most_used_sticker_count: i32,
    pub member_most_used_sticker: Option<MinimalMessage>,
}

#[derive(Serialize)]
pub struct MinimalMessage {
    pub id: i64,
    pub from: Option<String>,
    pub r#type: String,
    pub text: MessageText,
    pub date: DateTime<Utc>,
    pub duration_seconds: Option<i32>,
    pub discard_reason: Option<String>,
    pub file: Option<String>,
    pub media_type: Option<String>,
}

#[derive(Serialize)]
pub struct Streak {
    pub count: i32,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl From<Message> for MinimalMessage {
    fn from(value: Message) -> Self {
        Self {
            id: value.id,
            from: value.from,
            r#type: value.r#type,
            text: value.text,
            date: value.date,
            duration_seconds: value.duration_seconds,
            discard_reason: value.discard_reason,
            file: value.file,
            media_type: value.media_type,
        }
    }
}
