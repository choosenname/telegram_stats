use serde::Deserialize;

pub struct Config {
    #[allow(dead_code)]
    pub database_config: DatabaseConfig,
    pub app_config: AppConfig,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_config: envy::prefixed("DATABASE_").from_env()?,
            app_config: envy::prefixed("APP_").from_env()?,
        })
    }
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    #[allow(dead_code)]
    pub url: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub input_path: String,
    pub output_path: String,
    pub year: i32,
}
