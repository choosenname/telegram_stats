use std::fs::File;
use crate::config::Config;
use crate::core::types::chat::Chat;
use crate::data::models::data_getter::DataGetter;
use crate::data::models::data_preparer::DataPreparer;
use chrono::{TimeZone, Utc};

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

    println!("Message count {}", data_preparer.messages_count().await);
    
    // println!("First message {:?}", data_preparer.first_message().await.unwrap());
    save_to_json("./tmp/occurrences.json", &data_preparer.first_message().await.unwrap()).await;
    
    // println!("Occurrences {:?}", data_preparer.occurrences("я люблю тебя").await);
    save_to_json("./tmp/occurrences1.json", &data_preparer.occurrences("люблю тебя").await).await;
    save_to_json("./tmp/occurrences2.json", &data_preparer.occurrences("тебя люблю").await).await;
    
    // println!("Calls {:?}", data_preparer.calls().await);
    save_to_json("./tmp/calls.json", &data_preparer.calls().await).await;
}

async fn save_to_json(filename: &str, data: &impl serde::Serialize) {
    let file = File::create(filename).expect("Failed to create file");
    serde_json::to_writer(file, &data).expect("Failed to write JSON data");
}
