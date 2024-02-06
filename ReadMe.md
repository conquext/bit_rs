### WORK IN PROGRESS

# Bitcoin Utility CLI

Bitcoin Utility CLI is a command-line tool for performing various Bitcoin-related operations such as generating mnemonic keys, creating private keys, public keys, addresses, encoding/decoding hex, creating signatures, and more.

## Features

- **Generate Mnemonic Key:** Generate a random mnemonic key.
- **Generate Private Key:** Generate a random private key.
- **Create Public Key:** Derive a public key from a given private key.
- **Create Address:** Generate a Bitcoin address from a given public key.
- **Encode/Decode Hex:** Encode input into hex or decode hex input.
- **Create Signature:** Generate a signature for a given message using a private key.

## Getting Started

### Prerequisites

- Rust programming language and Cargo package manager installed.

### Installation

1. Clone the repository:

```bash
   git clone https://github.com/conquext/bit_rs.git
```

2. Navigate to the project directory:

```bash
    cd bit_rs
```

2. Build the project:

```bash
    cargo build --release
```

3. Run the executable with appropriate subcommands to perform Bitcoin-related operations:

```bash
    ./target/release/bit_rs <SUBCOMMAND> [OPTIONS]
```
