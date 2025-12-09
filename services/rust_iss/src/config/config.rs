use std::env;

#[derive(Clone)]
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
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            db_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),

            nasa_url: env::var("NASA_API_URL")
                .unwrap_or_else(|_| "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json".into()),

            nasa_key: env::var("NASA_API_KEY").unwrap_or_default(),

            iss_fallback_url: env::var("WHERE_ISS_URL")
                .unwrap_or_else(|_| "https://api.wheretheiss.at/v1/satellites/25544".into()),

            every_osdr: Self::get_u64("EVERY_OSDR", 600),
            every_iss: Self::get_u64("EVERY_ISS", 120),
            every_apod: Self::get_u64("EVERY_APOD", 43200),
            every_neo: Self::get_u64("EVERY_NEO", 7200),
            every_donki: Self::get_u64("EVERY_DONKI", 3600),
            every_spacex: Self::get_u64("EVERY_SPACEX", 3600),
        }
    }

    fn get_u64(key: &str, default: u64) -> u64 {
        env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
    }
}
