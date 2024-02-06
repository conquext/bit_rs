use bit_rs::{cli::setup::Cli, config::setup::Config};
use std::fs::File;

fn main() {
    let config_file: &'static str = "config.json";
    let config = Config::from_file(config_file);

    let mut log_file_path = config.log_file_path;
    if log_file_path.is_empty() {
        log_file_path = "bit_rs.log".to_string();
    }
    let mut log_file = File::create(log_file_path).expect("Failed to create log file");
    
    // Run the commands
    Cli::parse().print_or_write_files()
}