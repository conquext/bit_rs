use bt::{cli::setup::Cli, config::setup::Config, output::{Message, Output}};
use std::fs::File;

fn main() {
    let config_file: &'static str = "config.json";
    let config = Config::from_file(config_file);

    let mut log_file_path = config.log_file_path.clone();
    if log_file_path.is_empty() {
        log_file_path = "bit_rs.log".to_string();        
    }
    
    #[allow(unused_variable)]
    let log_file = File::create(log_file_path).expect("Failed to create log file");
    
    let mut cli = Cli::new(config.clone());

    // Run the commands
    match cli.handle_args() {
        Ok(msg) => match msg {
            Message::Empty => (),
            _ => println!("{}", Output::Message(msg)),
        },
        Err(err) => println!("{}", Output::Error(err)),
    }
}