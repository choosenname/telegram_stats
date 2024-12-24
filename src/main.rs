use crate::data::implementation::postgres::Postgres;
use crate::services::data_processor::DataProcessor;

pub mod services;
pub mod data;
pub mod utils;
pub mod core;

#[tokio::main]
async fn main() {
    let data_repository = Postgres;

    let processor = DataProcessor::new(data_repository);

    processor.process_file("/home/vlad/Dowloads/tg/result.json").await.unwrap();

}
