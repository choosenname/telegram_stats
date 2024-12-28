use crate::core::types::chat::{Chat, Message, MessageText, TextEntity};

pub struct DataPreparer {
    pub chat: Chat,
}

impl DataPreparer {
    pub fn new(chat: Chat) -> Self {
        Self { chat }
    }

    pub async fn retain_by_date(
        &mut self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) {
        self.chat
            .messages
            .retain(|message| message.date >= start && message.date <= end);
    }

    pub async fn messages_count(&self) -> usize {
        self.chat.messages.len()
    }

    pub async fn first_message(&self) -> Result<&Message, DataPreparerError> {
        match self.chat.messages.iter().min_by(|x, y| x.date.cmp(&y.date)) {
            None => Err(DataPreparerError::NoData),
            Some(message) => Ok(message),
        }
    }

    pub async fn occurrences(&self, search: &str) -> Vec<&Message> {
        self.chat
            .messages
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

        self.chat
            .messages
            .iter()
            .filter(|message| match &message.action {
                None => false,
                Some(action) => action == CALL_ACTION,
            })
            .collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataPreparerError {
    #[error("No data to prepare")]
    NoData,
}
