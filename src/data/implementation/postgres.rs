use crate::core::types::chat::{Chat, Message};
use crate::data::repositories::data_repository::DataRepository;
use crate::utils::errors::data_error::DataError;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct Postgres {
    pub pool: DatabaseConnection,
}

impl DataRepository for Postgres {
    type Data = Chat;

    async fn save(&self, data: Self::Data) -> Result<(), DataError> {
        use crate::core::entities::chats::ActiveModel as ChatModel;
        use crate::core::entities::chats::Entity as Chats;

        // Сохранение чата
        let chat_model = ChatModel {
            id: Set(data.id),
            name: Set(data.name.clone()),
            r#type: Set(data.r#type.clone()),
        };

        Chats::insert(chat_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(crate::core::entities::chats::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&self.pool)
            .await
            .map_err(|_| DataError::Save)?;

        // Сохранение сообщений
        self.save_messages(data.id, &data.messages).await?;

        Ok(())
    }
}

impl Postgres {
    pub async fn save_messages(&self, chat_id: i64, messages: &[Message]) -> Result<(), DataError> {
        use crate::core::entities::messages::ActiveModel as MessageModel;
        use crate::core::entities::messages::Entity as Messages;

        for message in messages {
            let message_model = MessageModel::from((chat_id, message.clone()));

            Messages::insert(message_model)
                .on_conflict(
                    sea_orm::sea_query::OnConflict::column(
                        crate::core::entities::messages::Column::Id,
                    )
                    .do_nothing()
                    .to_owned(),
                )
                .exec(&self.pool)
                .await
                .map_err(|_| DataError::Save)?;
        }

        Ok(())
    }
}
