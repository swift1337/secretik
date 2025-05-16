use anyhow::Result;
use clap::Parser;
use qrcode::{render::unicode, QrCode};

#[derive(Parser)]
/// Generate QR code
pub struct QRArgs {
    /// Text to encode in QR code
    #[arg(default_value_t = String::from(""))]
    pub text: String,
}

pub fn generate_qr(args: &QRArgs) -> Result<()> {
    let input = super::first_arg_or_stdin(args.text.clone())?;

    print_qr(input);

    Ok(())
}

pub fn print_qr(content: String) {
    let bytes = content.as_bytes();

    let code = QrCode::new(bytes).unwrap();

    let str = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{}", str);
}
