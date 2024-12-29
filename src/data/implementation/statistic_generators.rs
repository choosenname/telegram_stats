use crate::core::types::chat::{Chat, Message};
use crate::core::types::stats::{
    AdditionalMessagesStats, ChatStats, ConversationStats, MessagesStats,
};
use crate::data::models::data_preparer::DataPreparer;
use crate::data::repositories::statistic_repository::{
    Result, StatisticError, StatisticRepository,
};

impl StatisticRepository for ChatStats {
    type Data<'b> = &'b Chat;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(ChatStats {
            first_message: DataPreparer::first_message(&data.messages)
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

impl StatisticRepository for MessagesStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(MessagesStats {
            total_messages_count: data.len(),
            owner_messages_count: 0,
            member_messages_count: 0,
        })
    }
}

impl StatisticRepository for ConversationStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            first_message: DataPreparer::first_message_ref(&data)
                .await
                .map_err(StatisticError::FailedToGetData)?
                .clone()
                .into(),
            messages_stats: MessagesStats::get_stats(data).await?,
        })
    }
}
