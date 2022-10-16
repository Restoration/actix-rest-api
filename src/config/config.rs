use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
  databaseURL: String,
  #[serde(default="default_port")]
  port: u16,
}

fn default_port() -> u16 {
    8080
}
