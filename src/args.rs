use clap::{Parser, Subcommand};
use std::fmt;

#[derive(Parser)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Encrypt(EncryptArgs),
    Decrypt(DecryptArgs),
    Label(LabelArgs),
    QR(QRArgs),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Encrypt(_) => write!(f, "encrypt"),
            Command::Decrypt(_) => write!(f, "decrypt"),
            Command::Label(_) => write!(f, "label"),
            Command::QR(_) => write!(f, "qr"),
        }
    }
}

#[derive(Parser)]
#[clap(visible_aliases = ["e", "enc", "encode"])]
pub struct EncryptArgs {
    /// Text to encrypt
    pub text: String,
}

#[derive(Parser)]
#[clap(visible_aliases = ["d", "dec", "decode"])]
pub struct DecryptArgs {
    /// Text to decrypt
    pub text: String,
}

#[derive(Parser)]
#[clap(visible_aliases = ["l", "name" ])]
/// Generate random name
pub struct LabelArgs {
    /// Number of names to generate
    pub times: u32,
}

#[derive(Parser)]
/// Generate QR code
pub struct QRArgs {
    /// Text to encode in QR code
    pub text: String,
}
