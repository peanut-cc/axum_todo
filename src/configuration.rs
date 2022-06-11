

#[derive(serde::Deserialize,Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize,Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
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