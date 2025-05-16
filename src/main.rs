mod cli;
mod cmd;
mod crypt;

use clap::Parser;
use colored::Colorize;

const BANNER: &str = r#"
   _____                     __  _ __  
  / ___/___  _____________  / /_(_) /__
  \__ \/ _ \/ ___/ ___/ _ \/ __/ / //_/
 ___/ /  __/ /__/ /  /  __/ /_/ / ,<   
/____/\___/\___/_/   \___/\__/_/_/|_|  

Easy way to encrypt and decrypt your secrets
"#;

#[derive(Parser)]
#[command(bin_name = "secretik", about = BANNER)]
pub struct App {
    #[clap(subcommand)]
    pub command: cmd::Command,
}

fn main() {
    let cli = App::parse();

    let out = match cli.command {
        cmd::Command::Encrypt(ref args) => cmd::encrypt(args),
        cmd::Command::Decrypt(ref args) => cmd::decrypt(args),
        cmd::Command::Label(ref args) => cmd::generate_label(args),
        cmd::Command::QR(ref args) => cmd::generate_qr(args),
    };

    // success
    if out.is_ok() {
        return;
    }

    let err_message = format!("Command '{}' failed\n{}", cli.command, out.unwrap_err());

    println!("{}", err_message.red());

    std::process::exit(1);
}
