use sqlx::PgPool;
use crate::data::implementation::postgres::Postgres;
use crate::services::data_processor::DataProcessor;

mod services;
mod data;
mod utils;
mod core;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    
    let data_repository = Postgres{
        pool: PgPool::connect(env!("DATABASE_URL")).await.unwrap(),
    };

    let processor = DataProcessor::new(data_repository);

    processor.process_file("/home/vlad/Dowloads/tg/result.json").await.unwrap();

}
