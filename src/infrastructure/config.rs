use serde::Deserialize;

/// Configurazione dell'applicazione
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_environment")]
    pub environment: Environment,
    
    #[serde(default)]
    pub server: ServerConfig,
    
    #[serde(default)]
    pub database: DatabaseConfig,
    
    #[serde(default)]
    pub logging: LoggingConfig,
    
    #[serde(default)]
    pub features: FeatureFlags,
}

/// Environment di esecuzione
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

/// Configurazione del server HTTP
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
}

/// Configurazione del database
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_type")]
    pub db_type: String,
    
    pub connection_string: Option<String>,
    
    #[serde(default = "default_pool_size")]
    pub max_connections: u32,
}

/// Configurazione del logging
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    
    #[serde(default)]
    pub json_format: bool,
}

/// Feature flags per abilitare/disabilitare funzionalità
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FeatureFlags {
    #[serde(default)]
    pub enable_events: bool,
    
    #[serde(default)]
    pub enable_metrics: bool,
}

// Default functions
fn default_environment() -> Environment {
    Environment::Development
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_db_type() -> String {
    "in-memory".to_string()
}

fn default_pool_size() -> u32 {
    10
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: default_db_type(),
            connection_string: None,
            max_connections: default_pool_size(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            json_format: false,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logging: LoggingConfig::default(),
            features: FeatureFlags::default(),
        }
    }
}

impl AppConfig {
    /// Carica configurazione dalle variabili d'ambiente
    pub fn from_env() -> Self {
        let environment = std::env::var("ENVIRONMENT")
            .or_else(|_| std::env::var("ENV"))
            .ok()
            .and_then(|s| match s.to_lowercase().as_str() {
                "development" | "dev" => Some(Environment::Development),
                "staging" | "stage" => Some(Environment::Staging),
                "production" | "prod" => Some(Environment::Production),
                _ => None,
            })
            .unwrap_or(Environment::Development);

        Self {
            environment,
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or_else(|_| default_host()),
                port: std::env::var("SERVER_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(default_port),
            },
            database: DatabaseConfig {
                db_type: std::env::var("DATABASE_TYPE")
                    .unwrap_or_else(|_| default_db_type()),
                connection_string: std::env::var("DATABASE_URL").ok(),
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(default_pool_size),
            },
            logging: LoggingConfig {
                level: std::env::var("LOG_LEVEL")
                    .or_else(|_| std::env::var("RUST_LOG"))
                    .unwrap_or_else(|_| default_log_level()),
                json_format: std::env::var("LOG_JSON")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(false),
            },
            features: FeatureFlags {
                enable_events: std::env::var("FEATURE_EVENTS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(false),
                enable_metrics: std::env::var("FEATURE_METRICS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(false),
            },
        }
    }

    /// Restituisce l'indirizzo del server
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Verifica se siamo in ambiente di produzione
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    /// Verifica se siamo in ambiente di sviluppo
    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }
}
