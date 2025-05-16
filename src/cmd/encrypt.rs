use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use names::Generator as NamesGenerator;

use crate::{cli, cmd::qr, crypt};

#[derive(Parser)]
#[clap(visible_aliases = ["e", "enc", "encode"])]
pub struct EncryptArgs {
    /// Text to encrypt
    #[arg(default_value_t = String::from(""))]
    pub text: String,

    /// Write encrypted data to a file
    #[arg(short, long)]
    pub output: bool,

    /// Generate QR code
    #[arg(long)]
    pub qr: bool,
}

pub fn encrypt(args: &EncryptArgs) -> Result<()> {
    let input = super::first_arg_or_stdin(args.text.clone())?;

    // Prompt for password
    let password = cli::prompt_password(true, true)?;

    // Encrypt
    let encrypted = crypt::encrypt(input.as_bytes(), &password)?;
    let b64 = encrypted.to_base64();

    if !args.output {
        // just print to stdout
        println!("{}", b64);
    } else {
        let label = NamesGenerator::default().next().unwrap();
        let out = format!("{}.enc.txt", label);

        cli::write_to_file(&b64, &out)?;

        println!("Saved to {}", out.green());
    }

    // Print QR code if requested
    if args.qr {
        println!("\n{}\n", "QR code:".bold().green());
        qr::print_qr(b64);
    }

    Ok(())
}
