mod args;
mod crypt;

use clap::error::ErrorKind;
use clap::{Error, Parser};
use names::Generator as NamesGenerator;
use qrcode::QrCode;
use qrcode::render::unicode;

fn main() {
    let cli = args::CLI::parse();

    let out = match cli.command {
        args::Command::Encrypt(ref args) => encrypt(args),
        args::Command::Decrypt(ref args) => decrypt(args),
        args::Command::Name(ref args) => generate_name(args),
        args::Command::QR(ref args) => generate_qr(args),
    };

    if out.is_err() {
        println!("Command '{}' failed", cli.command.to_string());
        println!("{}", out.unwrap_err().render());
        std::process::exit(1);
    }

    // todo use stdin (optionally)
    // todo use stdin (optionally)
    // todo use file to write (optionally)
}

fn encrypt(args: &args::EncryptArgs) -> Result<(), Error> {
    const PASSWORD: &str = "qwerty";

    let _encrypted = crypt::encrypt(args.text.as_bytes(), PASSWORD).unwrap();

    Ok(())
}

fn decrypt(args: &args::DecryptArgs) -> Result<(), Error> {
    // todo
    println!("Calling decrypt({})", args.text);
    Ok(())
}

fn generate_name(args: &args::NameArgs) -> Result<(), Error> {
    if args.times <= 0 {
        return Err(err("times argument must be greater than zero."));
    }

    let mut generator = NamesGenerator::default();

    for _ in 0..args.times {
        println!("{}", generator.next().unwrap());
    }

    Ok(())
}

fn generate_qr(args: &args::QRArgs) -> Result<(), Error> {
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

fn err(message: &str) -> Error {
    Error::raw(ErrorKind::InvalidValue, message)
}
