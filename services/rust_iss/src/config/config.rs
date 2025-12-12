use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,

    pub fallback_iss_url: String,

    pub nasa_url: String,
    pub nasa_key: String,

    pub http_timeout: u64,
    pub port: u16,

    pub every_iss: u64,
    pub every_osdr: u64,
    pub every_apod: u64,
    pub every_neo: u64,
    pub every_donki: u64,
    pub every_spacex: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")?,

            fallback_iss_url: std::env::var("FALLBACK_ISS_URL")?,

            nasa_url: std::env::var("NASA_URL")?,
            nasa_key: std::env::var("NASA_KEY")?,

            http_timeout: std::env::var("HTTP_TIMEOUT")
                .unwrap_or("10".into())
                .parse()
                .unwrap_or(10),

            port: std::env::var("PORT")
                .unwrap_or("3000".into())
                .parse()
                .unwrap_or(3000),

            every_iss: std::env::var("EVERY_ISS").unwrap_or("60".into()).parse()?,
            every_osdr: std::env::var("EVERY_OSDR").unwrap_or("300".into()).parse()?,
            every_apod: std::env::var("EVERY_APOD").unwrap_or("3600".into()).parse()?,
            every_neo: std::env::var("EVERY_NEO").unwrap_or("3600".into()).parse()?,
            every_donki: std::env::var("EVERY_DONKI").unwrap_or("3600".into()).parse()?,
            every_spacex: std::env::var("EVERY_SPACEX").unwrap_or("600".into()).parse()?,
        })
    }
}
