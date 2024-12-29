use crate::data::models::data_preparer::DataPreparerError;

pub trait StatisticRepository<'a>: Sized {
    type Data<'b>;
    async fn get_stats(data: Self::Data<'a>) -> Result<Self>;
}

pub type Result<T> = core::result::Result<T, StatisticError>;

#[derive(Debug, thiserror::Error)]
pub enum StatisticError {
    #[error("Failed to generate statistic: {0}")]
    FailedToGenStat(String),
    #[error("Failed to get prepared data: {0}")]
    FailedToGetData(#[source] DataPreparerError),
}
