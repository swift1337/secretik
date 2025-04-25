mod args;

use clap::Parser;
use names::Generator as NamesGenerator;
use qrcode::QrCode;
use qrcode::render::unicode;

fn main() {
    let cli = args::CLI::parse();

    match cli.command {
        args::Command::Encrypt(args) => encrypt(args),
        args::Command::Decrypt(args) => decrypt(args),
        args::Command::Name(args) => generate_name(args),
        args::Command::QR(args) => generate_qr(args),
    };

    // todo handle errors
    // todo use stdin (optionally)
    // todo use file to write (optionally)
}

fn encrypt(args: args::EncryptArgs) {
    println!("Calling encrypt({})", args.text);
}

fn decrypt(args: args::DecryptArgs) {
    println!("Calling decrypt({})", args.text);
}

fn generate_name(args: args::NameArgs) {
    // todo error handling
    assert!(args.times > 0, "times must be greater than 0");

    let mut generator = NamesGenerator::default();

    for _ in 0..args.times {
        println!("{}", generator.next().unwrap());
    }
}

fn generate_qr(args: args::QRArgs) {
    print_qr(args.text);
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
