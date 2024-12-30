use crate::core::types::chat::{Chat, Message, MessageText, TextEntity};
use crate::core::types::stats::MinimalMessage;
use chrono::TimeDelta;
use regex::Regex;

type Result<T> = core::result::Result<T, DataPreparerError>;

pub struct DataPreparer;

impl DataPreparer {
    pub fn first_message<'a, I>(messages: I) -> Option<&'a Message>
    where
        I: Iterator<Item = &'a Message>,
    {
        match messages.min_by(|x, y| x.date.cmp(&y.date)) {
            None => None,
            Some(message) => Some(message),
        }
    }

    pub fn last_message<'a, I>(messages: I) -> Option<&'a Message>
    where
        I: Iterator<Item = &'a Message>,
    {
        match messages.max_by(|x, y| x.date.cmp(&y.date)) {
            None => None,
            Some(message) => Some(message),
        }
    }

    pub fn character_count<'a, I>(messages: I) -> Result<usize>
    where
        I: Iterator<Item = &'a Message>,
    {
        let mut total_characters = 0;

        for message in messages {
            match &message.text {
                MessageText::Plain(text) => {
                    total_characters += text.len();
                }
                MessageText::Entities(entities) => {
                    for entity in entities {
                        match entity {
                            TextEntity::Text(text) => {
                                total_characters += text.len();
                            }
                            TextEntity::Entity(entity) => {
                                total_characters += entity.text.len();
                            }
                        }
                    }
                }
            }
        }

        Ok(total_characters)
    }

    pub fn character_count_filtered<'a, I, F>(messages: I, mut filter: F) -> Result<usize>
    where
        I: Iterator<Item = &'a Message>,
        F: FnMut(&Message) -> bool,
    {
        Self::character_count(messages.filter(|message| filter(message)))
    }

    pub fn calls_durations<'a, I>(messages: I) -> Result<u32>
    where
        I: Iterator<Item = &'a Message>,
    {
        let mut duration = 0;

        for message in messages {
            if let Some(dur) = message.duration_seconds {
                duration += dur
            }
        }
        Ok(duration as u32)
    }

    pub fn longest_call<'a, I>(mut messages: I) -> Option<MinimalMessage>
    where
        I: Iterator<Item = &'a Message>,
    {
        let mut max_duration_message = None;
        let mut max_duration = 0;

        for message in messages {
            if let Some(dur) = message.duration_seconds {
                if dur > max_duration {
                    max_duration_message = Some(message);
                    max_duration = dur;
                }
            }
        }
        max_duration_message.map(|m| m.clone().into())
    }

    pub fn most_used_sticker<'a, I, F>(messages: I, mut filter: F) -> (i32, Option<MinimalMessage>)
    where
        I: Iterator<Item = &'a Message>,
        F: FnMut(&Message) -> bool,
    {
        let mut max_message = None;
        let mut max_used = 0;
        let mut usage_counter = std::collections::HashMap::new();

        for message in messages {
            if message.media_type == Some("sticker".to_string()) && filter(message) {
                if let Some(path) = &message.file {
                    let count = usage_counter.entry(path.clone()).or_insert(0);
                    *count += 1;
                    if *count > max_used {
                        max_used = *count;
                        max_message = Some(message.clone().into());
                    }
                }
            }
        }

        (max_used, max_message)
    }
}

impl Chat {
    pub async fn retain_by_date(
        &mut self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) {
        self.messages
            .retain(|message| message.date >= start && message.date <= end);
        self.messages.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn occurrences(&self, search: &Regex) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|message| match &message.text {
                MessageText::Plain(text) => search.is_match(text),
                MessageText::Entities(vec) => vec.iter().any(|item| match item {
                    TextEntity::Text(text) => search.is_match(text),
                    TextEntity::Entity(entity) => search.is_match(&entity.text),
                }),
            })
            .collect()
    }

    pub fn calls(&self) -> Vec<&Message> {
        const CALL_ACTION: &str = "phone_call";

        self.messages
            .iter()
            .filter(|message| match &message.action {
                None => false,
                Some(action) => action == CALL_ACTION,
            })
            .collect()
    }

    pub fn longest_conversation(&self) -> Vec<&Message> {
        let mut longest_conversation = vec![];
        let mut conversation = vec![];
        for message in self.messages.iter() {
            if conversation.is_empty() {
                conversation.push(message);
            } else {
                let time = message.date - conversation.last().unwrap().date;
                if time < TimeDelta::minutes(15) {
                    conversation.push(message);
                } else if conversation.len() > longest_conversation.len() {
                    longest_conversation = conversation.clone();
                } else {
                    conversation.clear();
                    conversation.push(message);
                }
            }
        }
        longest_conversation
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataPreparerError {
    #[error("No data to prepare")]
    NoData,
    #[error("Invalid calls durations in message: {id}")]
    InvalidCallsArray { id: i64 },
}
