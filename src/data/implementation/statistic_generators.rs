use crate::core::types::chat::{Chat, Message};
use crate::core::types::stats::{
    AdditionalMessagesStats, AllStats, CallsStats, ChatStats, ConversationStats, MessagesStats,
    OccurrencesStats,
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
                .map_err(StatisticError::FailedToGetData)?
                .clone()
                .into(),
            messages_stats: MessagesStats::get_stats(data.messages.iter().collect()).await?,
            additional_messages_stats: AdditionalMessagesStats::get_stats(&data.messages).await?,
        })
    }
}

impl StatisticRepository for MessagesStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(MessagesStats {
            total_messages_count: data.len(),
            owner_messages_count: data
                .iter()
                .filter(|message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user5769929151",
                })
                .count(),
            member_messages_count: data
                .iter()
                .filter(|message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user1150140845",
                })
                .count(),
        })
    }
}

impl StatisticRepository for AdditionalMessagesStats {
    type Data<'b> = &'b Vec<Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            total_characters_count: DataPreparer::character_count(data.iter())
                .map_err(StatisticError::FailedToGetData)?,
            owner_characters_count: DataPreparer::character_count_filtered(
                data.iter(),
                |message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user5769929151",
                },
            )
            .map_err(StatisticError::FailedToGetData)?,
            member_characters_count: DataPreparer::character_count_filtered(
                data.iter(),
                |message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user1150140845",
                },
            )
            .map_err(StatisticError::FailedToGetData)?,
        })
    }
}

impl StatisticRepository for ConversationStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            first_message: DataPreparer::first_message_ref(&data)
                .map_err(StatisticError::FailedToGetData)?
                .clone()
                .into(),
            last_message: DataPreparer::last_message_ref(&data)
                .map_err(StatisticError::FailedToGetData)?
                .clone()
                .into(),
            messages_stats: MessagesStats::get_stats(data).await?,
        })
    }
}

impl StatisticRepository for CallsStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        let duration =
            DataPreparer::calls_durations(data.iter().copied()).map_err(StatisticError::FailedToGetData)?;

        Ok(Self {
            total_calls_durations_sec: duration,
            total_calls_durations_min: duration / 60,
            longest_call_durations_min: DataPreparer::longest_call(data.iter().copied()),
        })
    }
}

impl StatisticRepository for OccurrencesStats {
    type Data<'b> = Vec<&'b Message>;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            first_message: DataPreparer::first_message_ref(&data)
                .map_err(StatisticError::FailedToGetData)?
                .clone()
                .into(),
            total_messages_count: data.len(),
            owner_messages_count: 0,
            member_messages_count: 0,
        })
    }
}

impl StatisticRepository for AllStats {
    type Data<'b> = Chat;

    async fn get_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            chat_stats: ChatStats::get_stats(&data).await?,
            occurrences: OccurrencesStats::get_stats(data.occurrences("люблю")).await?,
            longest_conversation: ConversationStats::get_stats(data.longest_conversation()).await?,
            calls_stats: CallsStats::get_stats(data.calls()).await?,
        })
    }
}
