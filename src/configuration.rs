
use deadpool_postgres::PoolConfig;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host:String,
    pub database_name: String,
    pub pool_max_size: u16
}

impl DatabaseSettings {

    pub fn with_db(&self) -> deadpool_postgres::Config {
        let mut pool_config = deadpool_postgres::Config::new();
        pool_config.user = Some(self.username.clone());
        pool_config.password = Some(self.password.clone());
        pool_config.dbname = Some(self.database_name.clone());
        pool_config.host = Some(self.host.clone());
        pool_config.port = Some(self.port.clone());
        pool_config.pool = Some(PoolConfig::new(self.pool_max_size.into()));
        pool_config
    }

}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::builder();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    settings = settings.add_source(config::File::from(configuration_directory.join("base")).required(true));
    settings
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
}