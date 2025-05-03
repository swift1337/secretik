mod decrypt;
mod encrypt;
mod label;
mod qr;

pub use decrypt::*;
pub use encrypt::*;
pub use label::*;
pub use qr::*;

use std::fmt;
use clap::Subcommand;
use anyhow::{Result, bail};

use crate::cli;

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

pub fn first_arg_or_stdin(value: String) -> Result<String> {
    if value.len() > 0 {
        return Ok(value);
    }

    let input = cli::read_stdin()?;

    if input.len() == 0 {
        bail!("No input provided");
    }

    Ok(input)
}
