#[derive(Debug, Clone)]
pub struct Wallets {
    path: String
}

impl Wallets {
    pub fn load(path: Option<String>) -> Self {
        Self {
            path: path.unwrap_or(format!("wallet01"))
        }
    }
}