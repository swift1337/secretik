use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Encrypt(EncryptArgs),
    Decrypt(DecryptArgs),
    Name(NameArgs),
    QR(QRArgs),
}

#[derive(Parser)]
#[clap(visible_aliases = ["e", "encode"])]
pub struct EncryptArgs {
    /// Text to encrypt
    pub text: String,
}

#[derive(Parser)]
#[clap(visible_aliases = ["d", "decode"])]
pub struct DecryptArgs {
    /// Text to decrypt
    pub text: String,
}

#[derive(Parser)]
#[clap(visible_aliases = ["n", "names", "label"])]
/// Generate random name
pub struct NameArgs {
    /// Number of names to generate
    pub times: u32,
}

#[derive(Parser)]
/// Generate QR code
pub struct QRArgs {
    /// Text to encode in QR code
    pub text: String,
}
