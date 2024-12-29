use crate::core::types::chat::{Message, MessageText};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct AllStats {
    pub chat_stats: ChatStats,
    pub occurrences: OccurrencesStats,
    pub longest_conversation: ConversationStats,
    pub calls_stats: CallsStats,
}

#[derive(Serialize)]
pub struct ChatStats {
    pub first_message: MinimalMessage,
    pub messages_stats: MessagesStats,
    pub additional_messages_stats: AdditionalMessagesStats,
}

#[derive(Serialize)]
pub struct MessagesStats {
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
pub struct OccurrencesStats {
    pub first_message: MinimalMessage,
    pub total_messages_count: usize,
    pub owner_messages_count: usize,
    pub member_messages_count: usize,
}

#[derive(Serialize)]
pub struct ConversationStats {
    pub first_message: MinimalMessage,
    pub last_message: MinimalMessage,
    pub messages_stats: MessagesStats,
}

#[derive(Serialize)]
pub struct CallsStats {
    pub total_calls_durations_sec: u32,
    pub total_calls_durations_min: u32,
    pub longest_call_durations_min: u32,
}

#[derive(Serialize)]
pub struct MinimalMessage {
    pub from: Option<String>,
    pub r#type: String,
    pub text: MessageText,
    pub date: DateTime<Utc>,
}

impl From<Message> for MinimalMessage {
    fn from(value: Message) -> Self {
        Self {
            from: value.from,
            r#type: value.r#type,
            text: value.text,
            date: value.date,
        }
    }
}
