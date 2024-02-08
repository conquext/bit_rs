use clap::{Parser, Subcommand, AppSettings};

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// create a new wallet
    #[clap(override_usage("bt new\n    bt new [wallet name]"))]
    #[clap(alias = "nw")]
    New {
        /// name for new wallet (to be created in the current folder)
        #[clap(value_parser, name = "wallet name")]
        name: String,
    },
    /// creates a new private key
    #[clap(override_usage("bt new-key"))]
    #[clap(alias = "nk")]
    NewKey,
    /// creates a new mnemonic seed
    #[clap(override_usage("bt new-seed"))]
    #[clap(alias = "ns")]
    NewSeed,
    /// open a wallet (from the current folder)
    #[clap(alias = "op")]
    Open {
        /// name of note to be opened
        #[clap(value_parser, name = "wallet name")]
        name: String,
    },
    /// list wallets in current folder
    #[clap(alias = "ls")]
    List {
        // list wallet(s) (or nt) | folder(s) (or fd)
        #[clap(value_enum, value_parser, name = "wallet type")]
        item_type: Option<String>,
    },
    /// decode a hex string into byte
    #[clap(alias = "de")]
    Decode {
        // decode a hex string into byte
        #[clap(value_parser, name = "hex string")]
        item_type: String,
    },
    /// encode a byte string into hex
    #[clap(alias = "en")]
    Encode {
        // encode a byte string into hex
        #[clap(value_parser, name = "encode byte into hex")]
        item_type: String,
    },
    /// 🆘 show this help message or help for given command.
    #[clap(alias = "h")]
    Help,
}

#[derive(Parser, Debug, Clone)]
#[clap(global_setting(AppSettings::HidePossibleValuesInHelp))]
#[clap(global_setting(AppSettings::DontCollapseArgsInUsage))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::ColorNever))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}