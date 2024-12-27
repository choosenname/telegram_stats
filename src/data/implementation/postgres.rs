use crate::core::types::chat::{Chat, Message};
use crate::data::repositories::data_repository::DataRepository;
use crate::utils::errors::data_error::DataError;

pub struct Postgres {
    pub pool: sqlx::PgPool,
}

impl DataRepository for Postgres {
    type Data = Chat;

    async fn save(&self, data: Self::Data) -> Result<(), DataError> {
        sqlx::query!(
            "INSERT INTO chats (id, name, type) VALUES ($1, $2, $3)\
            ON CONFLICT (id) DO NOTHING",
            data.id,
            data.name,
            data.r#type,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| DataError::Save)?;

        self.save_messages(data.id, &data.messages).await.unwrap();

        Ok(())
    }
}

impl Postgres {
    pub async fn save_messages(
        &self,
        chat_id: i64,
        messages: &[Message],
    ) -> Result<(), sqlx::Error> {
        for message in messages {
            sqlx::query!(
                r#"
                INSERT INTO messages (
                    id, chat_id, type, action, date, date_unixtime, from_user, from_id,
                    edited, edited_unixtime, reply_to_message_id, text, text_entities, members,
                    actor, actor_id, photo, width, height, file, thumbnail, self_destruct_period_seconds,
                    title, inviter, message_id, game_message_id, score, amount, currency, invoice_message_id,
                    recurring, duration_seconds, discard_reason, information_text, reason_app_id,
                    reason_app_name, reason_domain, values, to_id, to_user, distance, period,
                    schedule_date, emoticon, cost, months, new_title, new_icon_emoji_id, button_id,
                    peer_id, author, forwarded_from, saved_from, via_bot, media_type, performer,
                    mime_type, contact_information, contact_vcard, location_information,
                    live_location_period_seconds, place_name, address, game_title, game_description,
                    game_link, invoice_information, poll, gift_code, boost_peer_id, unclaimed,
                    via_giveaway, giveaway_information
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                    $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38,
                    $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56,
                    $57, $58, $59, $60, $61, $62, $63, $64, $65, $66, $67, $68, $69, $70, $71, $72, $73
                )
                ON CONFLICT (id) DO NOTHING
                "#,
                message.id,
                chat_id,
                message.r#type,
                message.action,
                message.date,
                message.date_unixtime,
                message.from,
                message.from_id,
                message.edited,
                message.edited_unixtime,
                message.reply_to_message_id,
                serde_json::to_value(&message.text).unwrap(),
                serde_json::to_value(&message.text_entities).unwrap(),
                serde_json::to_value(&message.members).unwrap(),
                message.actor,
                message.actor_id,
                message.photo,
                message.width,
                message.height,
                message.file,
                message.thumbnail,
                message.self_destruct_period_seconds,
                message.title,
                message.inviter,
                message.message_id,
                message.game_message_id,
                message.score,
                message.amount,
                message.currency,
                message.invoice_message_id,
                message.recurring,
                message.duration_seconds,
                message.discard_reason,
                message.information_text,
                message.reason_app_id,
                message.reason_app_name,
                message.reason_domain,
                serde_json::to_value(&message.values).unwrap(),
                message.to_id,
                message.to,
                message.distance,
                message.period,
                message.schedule_date,
                message.emoticon,
                message.cost,
                message.months,
                message.new_title,
                message.new_icon_emoji_id,
                message.button_id,
                message.peer_id,
                message.author,
                message.forwarded_from,
                message.saved_from,
                message.via_bot,
                message.media_type,
                message.performer,
                message.mime_type,
                serde_json::to_value(&message.contact_information).unwrap(),
                message.contact_vcard,
                serde_json::to_value(&message.location_information).unwrap(),
                message.live_location_period_seconds,
                message.place_name,
                message.address,
                message.game_title,
                message.game_description,
                message.game_link,
                serde_json::to_value(&message.invoice_information).unwrap(),
                serde_json::to_value(&message.poll).unwrap(),
                message.gift_code,
                message.boost_peer_id,
                message.unclaimed,
                message.via_giveaway,
                serde_json::to_value(&message.giveaway_information).unwrap()
            )
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
