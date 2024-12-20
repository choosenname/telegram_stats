use crate::data::repositories::data_repository::DataRepository;
use crate::utils::errors::data_error::DataError;

pub struct Postgres;

impl DataRepository for Postgres {
    type Data = ();

    async fn save(&self, data: Self::Data) -> Result<(), DataError> {
        todo!()
    }
}
