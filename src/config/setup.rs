use std::fs;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(PartialEq, Debug)]
pub enum ENV {
    Regtest,
    Testnet,
    Mainnet,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub data_dir: String,
    pub log_file_path: String,
    pub env: String,
}

impl Config {
    pub fn sample() -> Config {
        Config {
            data_dir: "./data".to_string(),
            log_file_path: "./log_file_path".to_string(),
            env: "testnet".to_string(),
        }
    }

    pub fn set<T>(&mut self, field: &str, value: T)
    where
        T: Into<String>,
    {
        match field {
            "env" => self.env = value.into(),
            "data_dir" => self.data_dir = value.into(),
            "log_file_path" => self.log_file_path = value.into(),
            _ => panic!("Invalid field name"),
        }
    }

    pub fn from_file(path: &'static str) -> Self {
        let config = fs::read_to_string(path).expect("Failed to read the config file");
        serde_json::from_str(&config).expect("Failed to parse the config file")
    }

    pub fn env(&self) -> ENV {
        match self.env.as_str() {
            "testnet" => ENV::Testnet,
            "regtest" => ENV::Regtest,
            "mainnet" => ENV::Mainnet,
            _ => ENV::Testnet
        }
    }
    pub fn is_mainnet(&self) -> bool {
        match self.env() {
            ENV::Mainnet => true,
            _ => false
        }
    }
}

lazy_static! {
    static ref CONFIG_FILE: &'static str = "config.json";
    pub static ref CONFIG: Config = Config::from_file(&CONFIG_FILE);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let mut config = Config {
            data_dir: "./test_data".to_string(),
            log_file_path: "./test_data".to_string(),
            env: "testnet".to_string(),
        };
        assert_eq!(config.env(), ENV::Testnet);

        config.set("env", "regtest");
        assert_eq!(config.env(), ENV::Regtest);
    }



    #[test]
    #[should_panic]
    fn test_from_file_missing_file() {
        Config::from_file("missing_config.json");
    }

    #[test]
    #[should_panic]
    fn test_from_file_malformed_json() {
        Config::from_file("config_malformed.json");
    }
}