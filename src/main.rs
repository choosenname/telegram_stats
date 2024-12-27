use crate::data::implementation::postgres::Postgres;
use crate::services::data_processor::DataProcessor;

mod core;
mod data;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let data_repository = Postgres {
        pool: sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    };

    let processor = DataProcessor::new(data_repository);

    processor
        .process_file_from_reader("/home/vlad/Dowloads/tg/result.json")
        .await
        .unwrap();
}
