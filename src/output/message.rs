use std::fmt::Display;

use crate::enums::VaultItem;

pub enum Message {
    WalletActivated(String),   
    WalletCreated(VaultItem, String),   
    OutputSaved(String),
    Text(String),   
    Empty,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Message::WalletActivated(name) => format!("activated \x1b[0;34m{name}\x1b[0m"),
                Message::Text(text) => format!("\x1b[0;34m{text}\x1b[0m"),
                Message::OutputSaved(path) => format!("Output saved to \x1b[0;34m{path}"),
                Message::WalletCreated(item_type, name) =>
                    format!("{} \x1b[0;34m{name}\x1b[0m created", item_type.full()),
                Message::Empty => "".to_string(),
            }
        )
    }
}