use crate::domain::types::chat::{Chat, Message, MessageText, TextEntity};
use crate::domain::types::stats::{MinimalMessage, Streak, WordCount};
use chrono::{Duration, TimeDelta, Utc};
use regex::Regex;

type Result<T> = core::result::Result<T, DataPreparerError>;

pub struct DataPreparer;

impl DataPreparer {
    fn message_text_to_string(text: &MessageText) -> String {
        match text {
            MessageText::Plain(text) => text.clone(),
            MessageText::Entities(entities) => {
                let mut result = String::new();
                for entity in entities {
                    match entity {
                        TextEntity::Text(text) => result.push_str(text),
                        TextEntity::Entity(entity) => result.push_str(&entity.text),
                    }
                }
                result
            }
        }
    }

    fn is_emoji_char(ch: char) -> bool {
        matches!(
            ch as u32,
            0x1F1E6..=0x1F1FF
                | 0x1F300..=0x1F5FF
                | 0x1F600..=0x1F64F
                | 0x1F680..=0x1F6FF
                | 0x1F700..=0x1F77F
                | 0x1F780..=0x1F7FF
                | 0x1F800..=0x1F8FF
                | 0x1F900..=0x1F9FF
                | 0x1FA00..=0x1FAFF
                | 0x2600..=0x26FF
                | 0x2700..=0x27BF
        )
    }

    fn extract_emojis(text: &str) -> Vec<String> {
        let mut emojis = Vec::new();
        let mut iter = text.chars().peekable();

        while let Some(ch) = iter.next() {
            if Self::is_emoji_char(ch) {
                let mut emoji = ch.to_string();
                if let Some('\u{FE0F}') = iter.peek() {
                    emoji.push(iter.next().unwrap());
                }
                emojis.push(emoji);
            }
        }

        emojis
    }

    pub fn top_emoji<'a, I>(messages: I) -> (Option<String>, i32)
    where
        I: Iterator<Item = &'a Message>,
    {
        let mut counts: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        for message in messages {
            let text = Self::message_text_to_string(&message.text);
            for emoji in Self::extract_emojis(&text) {
                *counts.entry(emoji).or_insert(0) += 1;
            }
        }

        counts
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(emoji, count)| (Some(emoji), count))
            .unwrap_or((None, 0))
    }

    pub fn top_words<'a, I>(messages: I, limit: usize) -> Vec<WordCount>
    where
        I: Iterator<Item = &'a Message>,
    {
        let word_re = Regex::new(r"[\p{L}\p{N}]+").unwrap();
        let stop_words: std::collections::HashSet<&'static str> = [
            "а",
            "да",
            "же",
            "за",
            "и",
            "из",
            "или",
            "к",
            "как",
            "на",
            "не",
            "ну",
            "о",
            "по",
            "про",
            "с",
            "со",
            "то",
            "у",
            "я",
            "мы",
            "ты",
            "вы",
            "он",
            "она",
            "оно",
            "они",
            "мне",
            "меня",
            "мной",
            "тебя",
            "тебе",
            "тобой",
            "нас",
            "нам",
            "вас",
            "вам",
            "его",
            "ее",
            "их",
            "мой",
            "моя",
            "мои",
            "твой",
            "твоя",
            "твои",
            "наш",
            "наша",
            "наши",
            "ваш",
            "ваша",
            "ваши",
            "это",
            "эта",
            "эти",
            "этот",
            "тот",
            "та",
            "те",
            "там",
            "тут",
            "здесь",
            "вот",
            "ли",
            "бы",
            "быть",
            "есть",
            "были",
            "был",
            "была",
            "будет",
            "буду",
            "будешь",
            "будем",
            "будете",
            "еще",
            "ещё",
            "уже",
            "если",
            "чтобы",
            "что",
            "кто",
            "когда",
            "где",
            "почему",
            "потом",
            "тогда",
            "сейчас",
            "сегодня",
            "вчера",
            "завтра",
            "очень",
            "просто",
            "вообще",
            "ладно",
            "ага",
            "в",
            "тяк",
            "так",
        ]
        .into_iter()
        .collect();
        let mut counts: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        for message in messages {
            let text = Self::message_text_to_string(&message.text);
            for word in word_re.find_iter(&text) {
                let value = word.as_str().to_lowercase();
                if value.len() < 2 || stop_words.contains(value.as_str()) {
                    continue;
                }
                *counts.entry(value).or_insert(0) += 1;
            }
        }

        let mut items: Vec<WordCount> = counts
            .into_iter()
            .map(|(word, count)| WordCount { word, count })
            .collect();
        items.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.word.cmp(&b.word)));
        items.truncate(limit);
        items
    }
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

    pub fn longest_call<'a, I>(messages: I) -> Option<MinimalMessage>
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
                let key = match &message.file {
                    Some(path) if !path.starts_with("(File not included") => Some(path.clone()),
                    _ => message.file_name.clone(),
                };

                if let Some(path) = key {
                    let count = usage_counter.entry(path).or_insert(0);
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

    pub fn message_streak<'a, I>(messages: I) -> Streak
    where
        I: Iterator<Item = &'a Message>,
    {
        let mut max_streak = Streak {
            count: 0,
            start: Utc::now().date_naive(),
            end: Utc::now().date_naive(),
        };

        let mut current_streak = Streak {
            count: 0,
            start: Utc::now().date_naive(),
            end: Utc::now().date_naive(),
        };

        let mut previous_date = None;

        for message in messages {
            let message_date = message.date.date_naive();

            match previous_date {
                None => {
                    // Инициализируем начальную дату
                    current_streak.start = message_date;
                    current_streak.end = message_date;
                    current_streak.count = 1;
                }
                Some(prev) if message_date == prev + Duration::days(1) => {
                    // Продолжаем последовательность
                    current_streak.end = message_date;
                    current_streak.count += 1;
                }
                Some(prev) if message_date > prev + Duration::days(1) => {
                    // Последовательность прервалась
                    if current_streak.count > max_streak.count {
                        max_streak = current_streak;
                    }
                    current_streak = Streak {
                        count: 1,
                        start: message_date,
                        end: message_date,
                    };
                }
                _ => {}
            }

            previous_date = Some(message_date);
        }

        // Проверяем последнюю последовательность
        if current_streak.count > max_streak.count {
            max_streak = current_streak;
        }

        max_streak
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
    #[allow(dead_code)]
    NoData,
    #[error("Invalid calls durations in message: {id}")]
    #[allow(dead_code)]
    InvalidCallsArray { id: i64 },
}
