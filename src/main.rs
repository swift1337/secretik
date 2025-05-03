mod args;
mod cli;
mod crypt;

use anyhow::{Result, bail};
use clap::Parser;
use names::Generator as NamesGenerator;
use qrcode::QrCode;
use qrcode::render::unicode;

fn main() {
    let cli = args::CLI::parse();

    let out = match cli.command {
        args::Command::Encrypt(ref args) => encrypt(args),
        args::Command::Decrypt(ref args) => decrypt(args),
        args::Command::Label(ref args) => generate_label(args),
        args::Command::QR(ref args) => generate_qr(args),
    };

    if out.is_err() {
        println!("Command '{}' failed", cli.command.to_string());
        println!("{}", out.unwrap_err());
        std::process::exit(1);
    }

    // todo use stdin (optionally)
    // todo use stdin (optionally)
    // todo use file to write (optionally)
}

fn encrypt(args: &args::EncryptArgs) -> Result<()> {
    let password = cli::utils::prompt_password(true, true)?;

    let encrypted = crypt::encrypt(args.text.as_bytes(), &password)?;

    println!("{}", encrypted.to_base64());

    Ok(())
}

fn decrypt(args: &args::DecryptArgs) -> Result<()> {
    let password = cli::utils::prompt_password(false, false)?;

    let decrypted_bytes = crypt::decrypt(&args.text, &password)?;
    let decrypted = String::from_utf8(decrypted_bytes)?;

    println!("{}", decrypted);

    Ok(())
}

fn generate_label(args: &args::LabelArgs) -> Result<()> {
    if args.times <= 0 {
        bail!("times argument must be greater than zero.");
    }

    let mut generator = NamesGenerator::default();

    for _ in 0..args.times {
        println!("{}", generator.next().unwrap());
    }

    Ok(())
}

fn generate_qr(args: &args::QRArgs) -> Result<()> {
    print_qr(args.text.clone());

    Ok(())
}

fn print_qr(content: String) {
    let bytes = content.as_bytes();

    let code = QrCode::new(bytes).unwrap();

    let str = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{}", str);
}
