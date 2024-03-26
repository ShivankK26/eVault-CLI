# eVault CLI

eVault CLI is a command-line tool built using Rust programming language that enables users to manage their passwords securely. With eVault CLI, users can add, view, edit, and store their passwords conveniently and safely.

## Features

- **Password Management**: Add, view, edit, and store passwords from the command line.
- **Security**: Passwords are stored securely using encryption to ensure data privacy.
- **Database**: Utilizes MySQL database for storing password entries.
- **Convenience**: Access and manage passwords directly from the terminal, making it easy to integrate into existing workflows.
- **Cross-Platform**: Works seamlessly on multiple platforms, including Linux, macOS, and Windows.

## Installation

To install eVault CLI, you need to have Rust installed on your system. Once Rust is installed, you can clone this repository and build the project using Cargo, Rust's package manager.

```bash
git clone https://github.com/ShivankK26/eVault-CLI.git
cargo build
```

After building the project, you can run the executable file generated in the `target/release` directory.

## Usage

### Adding a Password Entry

```bash
cargo run
```

This command will prompt you to enter the details for a new password entry, such as the website or service name, username, and password.

### Viewing Password Entries

```bash
cargo run
```

This command will display a list of all stored password entries.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on [GitHub](https://github.com/ShivankK26/eVault-CLI).
