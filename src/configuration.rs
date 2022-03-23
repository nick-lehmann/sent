use secrecy::{ExposeSecret, Secret};

pub enum Environment {
    Production,
    Local,
}

impl Environment {
    pub fn as_str(&self) -> String {
        match self {
            Environment::Production => "production",
            Environment::Local => "local",
        }
        .to_string()
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "production" => Ok(Environment::Production),
            "local" => Ok(Environment::Local),
            _ => Err(format!("Unable to convert {} into an environment", value)),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

impl ApplicationSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let mut config = config::Config::default();

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    config.merge(config::File::from(configuration_directory.join("base")))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    config.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    let settings: Configuration = config.try_into().unwrap();
    Ok(settings)
}
