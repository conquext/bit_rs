use clap::ValueEnum;


#[derive(ValueEnum, Clone, Debug)]
pub enum VaultItem {
    Wallet,
    W,
    Wallets,
    Ws,
}

impl VaultItem {
    pub fn full(&self) -> String {
        match self {
            VaultItem::Wallet | VaultItem::W => "wallet".to_string(),
            VaultItem::Wallets | VaultItem::Ws => "wallets".to_string(),
        }
    }
}