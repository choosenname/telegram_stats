use crate::data::repositories::data_repository::DataRepository;

pub struct GenStats<D: DataRepository> {
    pub data_repository: D,
}

impl<D: DataRepository> GenStats<D> {
    pub fn new(data_repository: D) -> Self {
        Self { data_repository }
    }
    
    
}
