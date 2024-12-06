use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub pg: deadpool_postgres::Config,
}
impl ServerConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        cfg.try_deserialize()
    }
}
