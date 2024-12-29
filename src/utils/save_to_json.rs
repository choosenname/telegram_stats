use std::fs::File;

pub async fn save_to_json(filename: &str, data: &impl serde::Serialize) {
    let file = File::create(filename).expect("Failed to create file");
    serde_json::to_writer(file, &data).expect("Failed to write JSON data");
}
