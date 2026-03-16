use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
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

    /// Encrypt plaintext from a file
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    pub file: Option<String>,

    /// Write encrypted data to a file
    #[arg(short, long)]
    pub output: bool,

    /// Generate QR code
    #[arg(long)]
    pub qr: bool,
}

pub fn encrypt(args: &EncryptArgs) -> Result<()> {
    if args.file.is_some() && args.output {
        bail!("The '--file' and '--output' flags cannot be used together");
    }

    // read input from file or stdin
    let input = if let Some(file_path) = args.file.as_deref() {
        cli::read_file(file_path)?
    } else {
        super::first_arg_or_stdin(args.text.clone())?
    };

    // prompt for password
    let password = cli::prompt_password(true, true)?;

    // encrypt
    let encrypted = crypt::encrypt(input.as_bytes(), &password)?;
    let b64 = encrypted.to_base64();

    if let Some(file_path) = args.file.as_deref() {
        let out = encrypted_output_path_for_file(file_path)?;
        let out = out.to_string_lossy().into_owned();

        cli::write_to_file(&b64, &out)?;

        println!("Saved to {}", out.green());
    } else if !args.output {
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

fn encrypted_output_path_for_file(input_path: &str) -> Result<PathBuf> {
    let input_path = Path::new(input_path);
    let input_filename = input_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid input file path"))?;

    let mut out_filename = input_path
        .file_stem()
        .unwrap_or(input_filename)
        .to_os_string();

    out_filename.push(".enc");

    if let Some(ext) = input_path.extension() {
        out_filename.push(".");
        out_filename.push(ext);
    }

    Ok(input_path.with_file_name(out_filename))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::encrypted_output_path_for_file;

    #[test]
    fn inserts_enc_before_extension() {
        assert_eq!(
            encrypted_output_path_for_file("notes.txt").unwrap(),
            PathBuf::from("notes.enc.txt")
        );
    }

    #[test]
    fn appends_enc_without_extension() {
        assert_eq!(
            encrypted_output_path_for_file("notes").unwrap(),
            PathBuf::from("notes.enc")
        );
    }

    #[test]
    fn preserves_parent_directory() {
        assert_eq!(
            encrypted_output_path_for_file("secrets/notes.txt").unwrap(),
            PathBuf::from("secrets/notes.enc.txt")
        );
    }
}
