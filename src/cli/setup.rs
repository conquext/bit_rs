use clap::Parser;

use crate::{btc_utils::key_gen::{generate_mnemonic_phrase, generate_private_key, generate_seed_from_mnemonic, Seed}, config::setup::Config, output::{error::Error, Message}, state::{args::{Args, Command}, wallets::Wallets}};

#[derive(Debug, Clone)]
pub struct Cli {
    args: Args,
    config: Config,
    wallets: Wallets,
    last_output: Option<String>,
}

impl Cli {
    pub fn new(config: Config) -> Self {
        Self {
            args: Args::parse(),
            config: config.clone(),
            wallets: Wallets::load(Some(config.data_dir)),
            last_output: None,
        }
    }
    
    pub fn handle_args(&mut self) -> Result<Message, Error> {
        match &self.args.command {  
            Command::New { name } => {
                // self.vaults.enter_vault(name)?;
                // Ok(Message::VaultEntered(name.to_owned()))
                Ok(Message::Empty)
            },        
            Command::NewPrivateKey => {
                let pk = generate_private_key();
                self.last_output = Some(pk.clone());
                Ok(Message::Text(format!("{}", pk)))
            },        
            Command::NewPhrase => {
                let mnemonic_phrase = generate_mnemonic_phrase();
                let bytes = generate_seed_from_mnemonic(&mnemonic_phrase, "");

                let seed_phrase = Seed {
                    bytes: bytes
                };

                println!("{} \n{:x}", mnemonic_phrase, seed_phrase);
                self.last_output = Some(mnemonic_phrase.clone());
                Ok(Message::Text(mnemonic_phrase))
            },
            Command::Save {path} => {
                match &self.last_output {
                    Some(_output) => {
                        Ok(Message::OutputSaved(format!("{}", path)))
                    },
                    _ => {
                        Ok(Message::Empty)
                    }
                }
            },
            Command::Print => {
                match &self.last_output {
                    Some(output) => {
                        Ok(Message::Text(format!("{}", output)))
                    },
                    _ => {
                        Ok(Message::Text(format!("No message in stdout")))
                    }
                }
            },
            Command::List { item_type } => {
                // self.vaults.ref_current()?.list(item_type);
                Ok(Message::Empty)
            },
            _ => Ok(Message::Empty),
        }        
    }
}