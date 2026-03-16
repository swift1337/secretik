use anyhow::{anyhow, Result};
use clap::Parser;
use qrcode::{render::unicode, types::QrError, QrCode};

#[derive(Parser)]
/// Generate QR code
pub struct QRArgs {
    /// Text to encode in QR code
    #[arg(default_value_t = String::from(""))]
    pub text: String,
}

pub fn generate_qr(args: &QRArgs) -> Result<()> {
    let input = super::first_arg_or_stdin(args.text.clone())?;

    print_qr(input)?;

    Ok(())
}

pub fn print_qr(content: String) -> Result<()> {
    let bytes = content.as_bytes();

    let code = QrCode::new(bytes).map_err(|err| match err {
        QrError::DataTooLong => anyhow!("QR content is too long to encode"),
        _ => anyhow!("Failed to generate QR code: {err}"),
    })?;

    let str = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{}", str);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::print_qr;

    #[test]
    fn returns_error_when_qr_content_is_too_long() {
        let very_long_text = "x".repeat(5000);
        let result = print_qr(very_long_text);
        assert!(result.is_err());
    }
}
