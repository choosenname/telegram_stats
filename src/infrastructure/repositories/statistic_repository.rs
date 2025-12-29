use crate::ingest::data_preparer::DataPreparerError;

pub trait StatisticRepository: Sized {
    type Data<'a>;
    async fn get_stats(data: Self::Data<'_>) -> Result<Self>;
}

pub type Result<T> = core::result::Result<T, StatisticError>;

#[derive(Debug, thiserror::Error)]
pub enum StatisticError {
    #[error("Failed to generate statistic: {0}")]
    #[allow(dead_code)]
    FailedToGenStat(String),
    #[error("Failed to get prepared data: {0}")]
    FailedToGetData(#[source] DataPreparerError),
}
