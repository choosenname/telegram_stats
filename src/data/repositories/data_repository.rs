use crate::utils::errors::data_error::DataError;

pub trait DataRepository {
    type Data;

    async fn save(&self, data: Self::Data) -> Result<(), DataError>;
}
