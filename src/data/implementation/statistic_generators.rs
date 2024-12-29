use crate::core::types::chat::{Chat, Message};
use crate::core::types::stats::{AdditionalMessagesStats, ChatStats, MessagesStats};
use crate::data::repositories::statistic_repository::{
    StatisticError, StatisticRepository, Result,
};

impl<'a> StatisticRepository<'a> for ChatStats {
    type Data<'b> = &'b Chat;

    async fn get_stats(data: Self::Data<'a>) -> Result<Self> {
        Ok(ChatStats {
            first_message: data
                .first_message()
                .await
                .map_err(StatisticError::FailedToGetData)?
                .clone()
                .into(),
            messages_stats: MessagesStats::get_stats(data.messages.iter().collect()).await?,
            additional_messages_stats: AdditionalMessagesStats {
                total_characters_count: 0,
                owner_characters_count: 0,
                member_characters_count: 0,
            },
        })
    }
}

impl<'a> StatisticRepository<'a> for MessagesStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'a>) -> Result<Self> {
        Ok(MessagesStats {
            total_messages_count: data.len(),
            owner_messages_count: 0,
            member_messages_count: 0,
        })
    }
}
