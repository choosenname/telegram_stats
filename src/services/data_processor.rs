use crate::data::repositories::data_repository::DataRepository;
use crate::data::repositories::statistic_repository::StatisticRepository;
use serde::Serialize;
use std::marker::{PhantomData};

pub struct DataProcessor<'a, D, T>
where
    D: DataRepository<T>,
    T: StatisticRepository<'a> + Serialize,
{
    pub data_repository: D,
    _phantom: PhantomData<&'a T>,
}

impl<'a, D, T> DataProcessor<'a, D, T>
where
    D: DataRepository<T>,
    T: StatisticRepository<'a> + Serialize,
{
    pub fn new(data_repository: D) -> Self {
        Self {
            data_repository,
            _phantom: PhantomData,
        }
    }

    pub async fn gen_stats_and_save(&self, data: T::Data<'a>) -> anyhow::Result<()> {
        let total_stats = T::get_stats(data).await?;
        self.data_repository.save(&total_stats).await?;
        Ok(())
    }
}
