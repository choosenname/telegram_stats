use crate::config::Config;
use crate::core::types::chat::{Chat, Message};
use crate::data::models::data_getter::DataGetter;
use crate::utils::save_to_json::save_to_json;
use chrono::{FixedOffset, TimeZone, Utc};
use crate::core::types::stats::{ChatStats, MessagesStats};
use crate::data::implementation::json::Json;
use crate::services::data_processor::DataProcessor;

mod config;
mod core;
mod data;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let config = Config::from_env().unwrap();
    
    let data_repository = Json::new(config.app_config.output_path);

    let data_processor: DataProcessor<Json, MessagesStats> = DataProcessor::new(data_repository);
    
    let mut data: Chat = DataGetter::process_file_from_reader(&config.app_config.input_path)
        .await
        .unwrap();
    
    data
        .retain_by_date(
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 0).unwrap(),
        )
        .await;
    
    // data_processor.gen_stats_and_save(data).await.unwrap();
    
    data_processor.gen_stats_and_save(data.occurrences("люблю").await).await.unwrap();

    save_to_json("./tmp/calls.json", &data.calls().await).await;

    save_to_json(
        "./tmp/conversion.json",
        &data.longest_conversation().await,
    )
    .await;
}
