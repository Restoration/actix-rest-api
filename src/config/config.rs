use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_port() -> u16 {
    8080
}
