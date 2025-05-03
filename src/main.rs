mod args;
mod cli;
mod crypt;

use anyhow::{Result, bail};
use clap::Parser;
use colored::Colorize;
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
        let err_message = format!(
            "Command '{}' failed\n{}",
            cli.command.to_string(),
            out.unwrap_err(),
        );

        println!("{}", err_message.red());

        std::process::exit(1);
    }

    // todo use file to write (optionally)
}

fn encrypt(args: &args::EncryptArgs) -> Result<()> {
    let input = first_arg_or_stdin(args.text.clone())?;

    // Prompt for password
    let password = cli::utils::prompt_password(true, true)?;

    // Encrypt
    let encrypted = crypt::encrypt(input.as_bytes(), &password)?;

    // Print encoded data
    let b64 = encrypted.to_base64();
    println!("{}", b64);

    // Print QR code if requested
    if args.qr {
        println!("\n{}\n", "QR code:".bold().green());
        print_qr(b64);
    }

    Ok(())
}

fn decrypt(args: &args::DecryptArgs) -> Result<()> {
    let input = first_arg_or_stdin(args.text.clone())?;

    let password = cli::utils::prompt_password(false, false)?;

    let decrypted_bytes = crypt::decrypt(&input, &password)?;
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
    let input = first_arg_or_stdin(args.text.clone())?;

    print_qr(input);

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

fn first_arg_or_stdin(value: String) -> Result<String> {
    if value.len() > 0 {
        return Ok(value);
    }

    let input = cli::utils::read_stdin()?;

    if input.len() == 0 {
        bail!("No input provided");
    }

    Ok(input)
}
