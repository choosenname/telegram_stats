use crate::application::services::data_processor::DataProcessor;
use crate::config::Config;
use crate::domain::types::chat::Chat;
use crate::domain::types::stats::AllStats;
use crate::infrastructure::persistence::json::Json;
use crate::ingest::data_getter::DataGetter;
use chrono::{TimeZone, Utc};

mod config;
mod application;
mod domain;
mod infrastructure;
mod ingest;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let config = Config::from_env().unwrap();

    let data_repository = Json::new(config.app_config.output_path);

    let data_processor = DataProcessor::new(data_repository);

    let mut data: Chat = DataGetter::process_file_from_reader(&config.app_config.input_path)
        .await
        .unwrap();

    data.retain_by_date(
        Utc.with_ymd_and_hms(config.app_config.year, 1, 1, 0, 0, 0)
            .unwrap(),
        Utc.with_ymd_and_hms(config.app_config.year, 12, 31, 23, 59, 0)
            .unwrap(),
    )
    .await;

    let source_dir = std::path::Path::new(&config.app_config.input_path)
        .parent()
        .and_then(|path| path.to_str())
        .unwrap_or(&config.app_config.input_path);

    data_processor
        .gen_stats_and_save::<AllStats>((&data, config.app_config.year, source_dir))
        .await
        .unwrap();
}
