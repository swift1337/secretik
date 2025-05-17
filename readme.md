<div align="center">
<picture>
    <img src="./.github/media/logo.png" height="180" alt="logo picture">
</picture>
    <h1>Secretik</h1>
    <p>Modern & opinionated cli tool for encrypting secrets</p>
</div>

## About

Secretik is a command-line tool encrypting sensitive information:

- Uses AES-256-GCM encryption with password-based key derivation (Argon2)
- Stores encrypted data in base64 for easy copying and sharing
- Can generate QR codes for encrypted data
- Provides a simple, secure way to protect sensitive information

## Installation

You can download the binary from the [Releases](https://github.com/swift1337/secretik/releases) page 
or install it via cargo:

```sh
cargo install secretik
```

## Usage

```sh
   _____                     __  _ __
  / ___/___  _____________  / /_(_) /__
  \__ \/ _ \/ ___/ ___/ _ \/ __/ / //_/
 ___/ /  __/ /__/ /  /  __/ /_/ / ,<
/____/\___/\___/_/   \___/\__/_/_/|_|

Easy way to encrypt and decrypt your secrets


Usage: secretik <COMMAND>

Commands:
  encrypt  [aliases: e, enc, encode]
  decrypt  [aliases: d, dec, decode]
  label    Generate random name [aliases: l, name]
  qr       Generate QR code
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Encryption Parameters

These parameters are chosen to provide a balance between security and performance, ensuring that the encryption is
robust against various types of attacks.

The parameters are fixed and not configurable to maintain simplicity and security.

**GCM Nonce Size**: 12 bytes

Argon2 parameters:

- **Salt Length**: 16 bytes
- **Iterations**: 3
- **Memory**: 64 MB
- **Threads**: 4
- **Key Length**: 32 bytes


### Commands

#### `encrypt`

Encrypts text using a password. Reads text from the first argument or stdin if no argument is provided.

```sh
secretik encrypt [TEXT] [OPTIONS]
```

**Options:**

*   `-o`, `--output`: Write the encrypted Base64 output to a file named `<random_name>.enc.txt` instead of printing to stdout.
*   `--qr`: Display a QR code of the encrypted Base64 output after encryption.


**Examples:**

```sh
# Encrypt text directly
secretik encrypt "my secret data"

# Encrypt text from stdin
echo "my secret data" | secretik encrypt

# Encrypt and save to file
secretik encrypt "my secret data" -o

# Encrypt and show QR code
secretik encrypt "my secret data" --qr
```

#### `decrypt`

Decrypts Base64 encoded text using a password. Reads text from the first argument or stdin by default.

```sh
secretik decrypt [TEXT_OR_FILE] [OPTIONS]
```

**Options:**

*   `-f`, `--from-file`: Read the encrypted Base64 text from the specified file path instead of the argument or stdin.

**Examples:**

```sh
# Decrypt text directly
secretik decrypt "..." # Paste Base64 encoded text here

# Decrypt text from stdin
echo "..." | secretik decrypt

# Decrypt from a file
secretik decrypt -f my_secret.enc.txt
```

#### `label`

Generates one or more random names (adjective-noun format).

```sh
secretik label <TIMES>
```

**Arguments:**

*   `TIMES`: The number of names to generate.

**Example:**

```sh
# Generate 3 random names
secretik label 3
```

#### `qr`

Generates a QR code for the given text. Reads text from the first argument or stdin if no argument is provided.

```sh
secretik qr [TEXT]
```

**Example:**

```sh
# Generate QR code for text
secretik qr "https://example.com"

# Generate QR code from stdin
echo "Some data" | secretik qr
```
