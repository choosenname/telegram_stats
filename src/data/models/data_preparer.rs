use crate::core::types::chat::{Chat, Message, MessageText, TextEntity};
use chrono::TimeDelta;

type Result<T> = core::result::Result<T, DataPreparerError>;

pub struct DataPreparer;

impl DataPreparer {
    pub async fn first_message_ref<'a>(messages: &[&'a Message]) -> Result<&'a Message> {
        match messages.iter().min_by(|x, y| x.date.cmp(&y.date)) {
            None => Err(DataPreparerError::NoData),
            Some(message) => Ok(message),
        }
    }

    pub async fn first_message(messages: &[Message]) -> Result<&Message> {
        match messages.iter().min_by(|x, y| x.date.cmp(&y.date)) {
            None => Err(DataPreparerError::NoData),
            Some(message) => Ok(message),
        }
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

    

    pub async fn occurrences(&self, search: &str) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|message| match &message.text {
                MessageText::Plain(text) => text.contains(search),
                MessageText::Entities(vec) => vec.iter().any(|item| match item {
                    TextEntity::Text(text) => text.contains(search),
                    TextEntity::Entity(entity) => entity.text.contains(search),
                }),
            })
            .collect()
    }

    pub async fn calls(&self) -> Vec<&Message> {
        const CALL_ACTION: &str = "phone_call";

        self.messages
            .iter()
            .filter(|message| match &message.action {
                None => false,
                Some(action) => action == CALL_ACTION,
            })
            .collect()
    }

    pub async fn longest_conversation(&self) -> Vec<&Message> {
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
}
