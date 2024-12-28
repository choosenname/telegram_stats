use crate::config::Config;
use crate::core::types::chat::Chat;
use crate::data::models::data_getter::DataGetter;
use crate::data::models::data_preparer::DataPreparer;
use crate::data::repositories::data_repository::DataRepository;
use chrono::{TimeZone, Utc};
use serde::de::DeserializeOwned;

mod config;
mod core;
mod data;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let config = Config::from_env().unwrap();

    // let data_repository = Postgres {
    //     pool: sea_orm::Database::connect(config.database_config.url)
    //         .await
    //         .unwrap(),
    // };
    //
    // let data_processor = DataProcessor::new(data_repository);

    let data: Chat = DataGetter::process_file_from_reader(&config.app_config.path)
        .await
        .unwrap();

    let mut data_preparer = DataPreparer::new(data);
    data_preparer
        .retain_by_date(
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 0).unwrap(),
        )
        .await;

    print!("Message count {}", data_preparer.messages_count().await);
    print!(
        "First message {:?}",
        data_preparer.first_message().await.unwrap()
    );
}
