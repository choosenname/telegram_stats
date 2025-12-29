use crate::infrastructure::repositories::data_repository::DataRepository;
use crate::infrastructure::repositories::statistic_repository::StatisticRepository;
use serde::Serialize;

pub struct DataProcessor<D>
where
    D: DataRepository,
{
    pub data_repository: D,
}

impl<D> DataProcessor<D>
where
    D: DataRepository,
{
    pub fn new(data_repository: D) -> Self {
        Self { data_repository }
    }

    pub async fn gen_stats_and_save<'a, T>(&self, data: T::Data<'a>) -> anyhow::Result<()>
    where
        T: StatisticRepository + Serialize,
    {
        let total_stats = T::get_stats(data).await?;
        self.data_repository.save(&total_stats).await?;
        Ok(())
    }
}
