use crate::domain::types::chat::{Chat, Message};
use crate::infrastructure::repositories::storage_repository::{StorageError, StorageRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};

#[allow(dead_code)]
pub struct Postgres {
    pub pool: DatabaseConnection,
}

impl StorageRepository for Postgres {
    type Data = Chat;

    async fn save(&self, data: Self::Data) -> Result<(), StorageError> {
        use crate::domain::entities::chats::ActiveModel as ChatModel;
        use crate::domain::entities::chats::Entity as Chats;

        // Сохранение чата
        let chat_model = ChatModel {
            id: Set(data.id),
            name: Set(data.name.clone()),
            r#type: Set(data.r#type.clone()),
        };

        Chats::insert(chat_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(crate::domain::entities::chats::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&self.pool)
            .await
            .map_err(|_| StorageError::Save)?;

        // Сохранение сообщений
        self.save_messages(data.id, &data.messages).await?;

        Ok(())
    }
}

impl Postgres {
    #[allow(dead_code)]
    pub async fn save_messages(&self, chat_id: i64, messages: &[Message]) -> Result<(), StorageError> {
        use crate::domain::entities::messages::ActiveModel as MessageModel;
        use crate::domain::entities::messages::Entity as Messages;

        for message in messages {
            let message_model =     MessageModel::from((chat_id, message.clone()));

            Messages::insert(message_model)
                .on_conflict(
                    sea_orm::sea_query::OnConflict::column(
                        crate::domain::entities::messages::Column::Id,
                    )
                    .do_nothing()
                    .to_owned(),
                )
                .exec(&self.pool)
                .await
                .map_err(|_| StorageError::Save)?;
        }

        Ok(())
    }
}
