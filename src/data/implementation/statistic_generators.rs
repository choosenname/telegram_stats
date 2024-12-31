use crate::core::types::chat::{Chat, Message};
use crate::core::types::stats::{
    AdditionalMessagesStats, AllStats, CallsStats, ChatStats, MessagesStats, MostUsedSticker,
};
use crate::data::models::data_preparer::DataPreparer;
use crate::data::repositories::statistic_repository::{
    Result, StatisticError, StatisticRepository,
};
use regex::Regex;

impl StatisticRepository for ChatStats {
    type Data<'b> = &'b ChatStatData<'b>;

    async fn gen_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(ChatStats {
            messages_stats: MessagesStats::gen_stats(MessageStatData {
                messages: data.chat.messages.iter().collect(),
                owner_id: data.owner_id,
            })
            .await?,
            additional_messages_stats: AdditionalMessagesStats::gen_stats(MessageStatData {
                messages: data.chat.messages.iter().collect(),
                owner_id: data.owner_id,
            })
            .await?,
        })
    }
}

impl StatisticRepository for MessagesStats {
    type Data<'a> = MessageStatData<'a>;

    async fn gen_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            first_message: DataPreparer::first_message(data.messages.iter().copied())
                .map(|message| message.clone().into()),
            last_message: DataPreparer::last_message(data.messages.iter().copied())
                .map(|message| message.clone().into()),
            total_messages_count: data.messages.len(),
            owner_messages_count: data
                .messages
                .iter()
                .filter(|message| match &message.from_id {
                    None => false,
                    Some(id) => id == data.owner_id,
                })
                .count(),
            member_messages_count: data
                .messages
                .iter()
                .filter(|message| match &message.from_id {
                    None => false,
                    Some(id) => id != data.owner_id,
                })
                .count(),
        })
    }
}

impl StatisticRepository for AdditionalMessagesStats {
    type Data<'b> = MessageStatData<'b>;

    async fn gen_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            total_characters_count: DataPreparer::character_count(data.messages.iter().copied())
                .map_err(StatisticError::FailedToGetData)?,
            owner_characters_count: DataPreparer::character_count_filtered(
                data.messages.iter().copied(),
                |message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user5769929151",
                },
            )
            .map_err(StatisticError::FailedToGetData)?,
            member_characters_count: DataPreparer::character_count_filtered(
                data.messages.iter().copied(),
                |message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user1150140845",
                },
            )
            .map_err(StatisticError::FailedToGetData)?,
        })
    }
}

impl StatisticRepository for CallsStats {
    type Data<'a> = MessageStatData<'a>;

    async fn gen_stats(data: Self::Data<'_>) -> Result<Self> {
        let duration = DataPreparer::calls_durations(data.messages.iter().copied())
            .map_err(StatisticError::FailedToGetData)?;

        Ok(Self {
            total_calls_durations_sec: duration,
            total_calls_durations_min: duration / 60,
            longest_call_durations_min: DataPreparer::longest_call(data.messages.iter().copied()),
        })
    }
}

impl StatisticRepository for MostUsedSticker {
    type Data<'b> = MessageStatData<'b>;

    async fn gen_stats(data: Self::Data<'_>) -> Result<Self> {
        let owner =
            DataPreparer::most_used_sticker(
                data.messages.iter().copied(),
                |message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user5769929151",
                },
            );

        let member =
            DataPreparer::most_used_sticker(
                data.messages.iter().copied(),
                |message| match &message.from_id {
                    None => false,
                    Some(id) => id == "user1150140845",
                },
            );

        Ok(Self {
            owner_most_used_sticker_count: owner.0,
            owner_most_used_sticker: owner.1,
            member_most_used_sticker_count: member.0,
            member_most_used_sticker: member.1,
        })
    }
}

impl StatisticRepository for AllStats {
    type Data<'a> = ChatStatData<'a>;

    async fn gen_stats(data: Self::Data<'_>) -> Result<Self> {
        Ok(Self {
            chat_stats: ChatStats::gen_stats(&data).await?,
            occurrences: MessagesStats::gen_stats(MessageStatData {
                messages: data.chat.occurrences(
                    &Regex::new(r"(?i)\bлюблю\b.*\bтебя\b|\bтебя\b.*\bлюблю\b|\bи я тебя\b")
                        .unwrap(),
                ),
                owner_id: data.owner_id,
            })
            .await?,
            longest_conversation: MessagesStats::gen_stats(MessageStatData {
                messages: data.chat.longest_conversation(),
                owner_id: data.owner_id,
            })
            .await?,
            calls_stats: CallsStats::gen_stats(MessageStatData {
                messages: data.chat.calls(),
                owner_id: data.owner_id,
            })
            .await?,
            most_used_sticker: MostUsedSticker::gen_stats(MessageStatData {
                messages: data.chat.messages.iter().collect(),
                owner_id: data.owner_id,
            })
            .await?,
        })
    }
}

pub struct ChatStatData<'a> {
    pub chat: &'a Chat,
    pub owner_id: &'a str,
}

pub struct MessageStatData<'a> {
    pub messages: Vec<&'a Message>,
    pub owner_id: &'a str,
}
