use crate::data::implementation::postgres::Postgres;
use crate::services::data_processor::DataProcessor;
use sqlx::PgPool;

mod core;
mod data;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let data_repository = Postgres {
        pool: PgPool::connect(env!("DATABASE_URL")).await.unwrap(),
    };

    let processor = DataProcessor::new(data_repository);

    processor
        .process_file_from_reader("/home/vlad/Dowloads/tg/result.json")
        .await
        .unwrap();
}
