use crate::data::repositories::data_repository::{DataRepository, Result};
use crate::utils::save_to_json::save_to_json;
use serde::Serialize;

pub struct Json {
    pub path: String,
}

impl Json {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl<T: Serialize> DataRepository<T> for Json {
    async fn save(&self, data: &T) -> Result<()> {
        save_to_json(&self.path, data).await;
        Ok(())
    }
}
