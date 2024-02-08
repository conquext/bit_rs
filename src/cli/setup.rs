use clap::Parser;

use crate::{config::setup::Config, output::{error::Error, Message}, state::{args::{Args, Command}, wallets::Wallets}};

#[derive(Debug, Clone)]
pub struct Cli {
    args: Args,
    config: Config,
    wallets: Wallets,
}

impl Cli {
    pub fn new(config: Config) -> Self {
        Self {
            args: Args::parse(),
            config: config.clone(),
            wallets: Wallets::load(Some(config.data_dir)),
        }
    }
    
    pub fn handle_args(&mut self) -> Result<Message, Error> {
        match &self.args.command {  
            Command::New { name } => {
                // self.vaults.enter_vault(name)?;
                // Ok(Message::VaultEntered(name.to_owned()))
                Ok(Message::Empty)
            },          
            Command::List { item_type } => {
                // self.vaults.ref_current()?.list(item_type);
                Ok(Message::Empty)
            },
            _ => Ok(Message::Empty),
        }        
    }
}