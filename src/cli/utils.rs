use std::io::IsTerminal;
use std::io::Write;
use std::io::prelude::*;

use anyhow::Result;
use anyhow::bail;
use rpassword::read_password;

pub fn prompt_password(strong: bool, confirm: bool) -> Result<String> {
    print!("Password: ");
    std::io::stdout().flush()?;

    let password = read_password()?;

    if strong && !validate_password(&password) {
        return Err(anyhow::anyhow!(
            "Weak pass. Should contain 12+ chars and digits"
        ));
    }

    if !confirm {
        return Ok(password);
    }

    print!("Repeat password: ");
    std::io::stdout().flush()?;

    let confirmation = read_password()?;

    if password != confirmation {
        return Err(anyhow::anyhow!("passwords do not match"));
    }

    Ok(password)
}

fn validate_password(password: &str) -> bool {
    if password.len() < 12 {
        return false;
    }

    let mut has_char = false;
    let mut has_digit = false;

    for char in password.chars() {
        if char.is_ascii_alphabetic() {
            has_char = true;
        } else if char.is_ascii_digit() {
            has_digit = true;
        }
    }

    has_char && has_digit
}

pub fn read_stdin() -> Result<String> {
    let stdin = std::io::stdin();

    // if interactive user terminal, fail
    if stdin.is_terminal() {
        bail!("No input provided");
    }

    let mut str = String::new();

    for line_result in stdin.lock().lines() {
        let line = line_result?;
        str.push_str(&line);
    }

    Ok(str)
}
