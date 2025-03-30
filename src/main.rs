mod args;

use clap::Parser;

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
    println!("Calling generate_name(times={})", args.times);
}

fn generate_qr(args: args::QRArgs) {
    println!("Calling generate_qr(text={})", args.text);
}
