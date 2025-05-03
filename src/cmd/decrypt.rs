use anyhow::Result;
use clap::Parser;

use crate::{cli, crypt};

use super::first_arg_or_stdin;

#[derive(Parser)]
#[clap(visible_aliases = ["d", "dec", "decode"])]
pub struct DecryptArgs {
    /// Text to decrypt
    #[arg(default_value_t = String::from(""))]
    pub text_or_file: String,

    /// Decrypt from a file instead of stdin
    #[arg(short, long)]
    pub from_file: bool,
}

pub fn decrypt(args: &DecryptArgs) -> Result<()> {
    let input = if !args.from_file {
        first_arg_or_stdin(args.text_or_file.clone())?
    } else {
        cli::read_file(&args.text_or_file)?
    };

    let input = input.trim();

    let password = cli::prompt_password(false, false)?;

    let decrypted_bytes = crypt::decrypt(&input, &password)?;
    let decrypted = String::from_utf8(decrypted_bytes)?;

    println!("{}", decrypted);

    Ok(())
}
