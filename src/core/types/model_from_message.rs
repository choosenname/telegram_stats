use crate::core::entities::messages::ActiveModel;
use crate::core::types::chat::Message;
use sea_orm::ActiveValue::Set;

impl From<(i64, Message)> for ActiveModel {
    fn from(value: (i64, Message)) -> Self {
        let (chat_id, message) = value;
        Self {
            id: Set(message.id),
            chat_id: Set(chat_id),
            r#type: Set(message.r#type),
            action: Set(message.action),
            date: Set(message.date.to_string()),
            date_unixtime: Set(message.date_unixtime),
            from_user: Set(message.from),
            from_id: Set(message.from_id),
            edited: Set(message.edited),
            edited_unixtime: Set(message.edited_unixtime),
            reply_to_message_id: Set(message.reply_to_message_id),
            text: Set(serde_json::to_value(message.text).unwrap_or_default()),
            text_entities: Set(serde_json::to_value(message.text_entities).unwrap_or_default()),
            members: Set(message
                .members
                .map(|m| serde_json::to_value(m).ok().unwrap_or_default())),
            actor: Set(message.actor),
            actor_id: Set(message.actor_id),
            photo: Set(message.photo),
            width: Set(message.width),
            height: Set(message.height),
            file: Set(message.file),
            thumbnail: Set(message.thumbnail),
            self_destruct_period_seconds: Set(message.self_destruct_period_seconds),
            title: Set(message.title),
            inviter: Set(message.inviter),
            message_id: Set(message.message_id),
            game_message_id: Set(message.game_message_id),
            score: Set(message.score),
            amount: Set(message.amount),
            currency: Set(message.currency),
            invoice_message_id: Set(message.invoice_message_id),
            recurring: Set(message.recurring),
            duration_seconds: Set(message.duration_seconds),
            discard_reason: Set(message.discard_reason),
            information_text: Set(message.information_text),
            reason_app_id: Set(message.reason_app_id),
            reason_app_name: Set(message.reason_app_name),
            reason_domain: Set(message.reason_domain),
            values: Set(message
                .values
                .map(|v| serde_json::to_value(v).ok().unwrap_or_default())),
            to_id: Set(message.to_id),
            to_user: Set(message.to),
            distance: Set(message.distance),
            period: Set(message.period),
            schedule_date: Set(message.schedule_date),
            emoticon: Set(message.emoticon),
            cost: Set(message.cost),
            months: Set(message.months),
            new_title: Set(message.new_title),
            new_icon_emoji_id: Set(message.new_icon_emoji_id),
            button_id: Set(message.button_id),
            peer_id: Set(message.peer_id),
            author: Set(message.author),
            forwarded_from: Set(message.forwarded_from),
            saved_from: Set(message.saved_from),
            via_bot: Set(message.via_bot),
            media_type: Set(message.media_type),
            performer: Set(message.performer),
            mime_type: Set(message.mime_type),
            contact_information: Set(message
                .contact_information
                .map(|c| serde_json::to_value(c).ok().unwrap_or_default())),
            contact_vcard: Set(message.contact_vcard),
            location_information: Set(message
                .location_information
                .map(|l| serde_json::to_value(l).ok().unwrap_or_default())),
            live_location_period_seconds: Set(message.live_location_period_seconds),
            place_name: Set(message.place_name),
            address: Set(message.address),
            game_title: Set(message.game_title),
            game_description: Set(message.game_description),
            game_link: Set(message.game_link),
            invoice_information: Set(message
                .invoice_information
                .map(|i| serde_json::to_value(i).ok().unwrap_or_default())),
            poll: Set(message
                .poll
                .map(|p| serde_json::to_value(p).ok().unwrap_or_default())),
            gift_code: Set(message.gift_code),
            boost_peer_id: Set(message.boost_peer_id),
            unclaimed: Set(message.unclaimed),
            via_giveaway: Set(message.via_giveaway),
            giveaway_information: Set(message
                .giveaway_information
                .map(|g| serde_json::to_value(g).ok().unwrap_or_default())),
        }
    }
}
