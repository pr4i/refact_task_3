use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub nasa_url: String,
    pub nasa_key: String,
    pub iss_fallback_url: String,

    pub every_osdr: u64,
    pub every_iss: u64,
    pub every_apod: u64,
    pub every_neo: u64,
    pub every_donki: u64,
    pub every_spacex: u64,

    pub http_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            db_url: env("DATABASE_URL", "postgres://postgres:postgres@db/postgres"),
            nasa_url: env(
                "NASA_API_URL",
                "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json",
            ),
            nasa_key: env("NASA_API_KEY", ""),
            iss_fallback_url: env(
                "WHERE_ISS_URL",
                "https://api.wheretheiss.at/v1/satellites/25544",
            ),
            every_osdr: env_u64("FETCH_EVERY_SECONDS", 600),
            every_iss: env_u64("ISS_EVERY_SECONDS", 120),
            every_apod: env_u64("APOD_EVERY_SECONDS", 43200),
            every_neo: env_u64("NEO_EVERY_SECONDS", 7200),
            every_donki: env_u64("DONKI_EVERY_SECONDS", 3600),
            every_spacex: env_u64("SPACEX_EVERY_SECONDS", 3600),

            http_timeout: Duration::from_secs(20),
        }
    }
}

fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}
