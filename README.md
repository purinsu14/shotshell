# Shotshell (shsh)

Shotshell (shsh) is a fast and simple shell written in Rust. It provides a simple yet functional command-line interface (CLI) for interacting with the operating system.

## Features

- Supports basic shell commands like `cd`, `exit`, and custom command execution.
- Displays the current working directory.
- Handles command errors gracefully.

## Installation

To use Shotshell, you'll need to have Rust installed on your system. You can download Rust from the [official website](https://www.rust-lang.org/tools/install).

Once you have Rust set up, you can clone the repository and build the project:

```bash
git clone https://github.com/purinsu14/shotshell.git
cd shotshell
```

Then using make:
> **NOTE:** Linux only. Currently using ~/.local/bin convention.
```bash
make install
```

Or using cargo:
```bash
cargo install --path .
```

## Usage

To start Shotshell, simply run the executable, anywhere on your system:

```bash
shsh
```

You should see the initial prompt `$>`. From here, you can enter various commands:

- `cd <directory>`: Change the current working directory.
- `exit`: Exit the Shotshell.
- Any other command will be executed as a subprocess.

## Uninstalling

To uninstall Shotshell, run the following command:

Using make:
```bash
make uninstall
```

Using cargo:
```bash
cargo uninstall shotshell
```

This will remove the Shotshell executable from your system.

## License

Shotshell is licensed under the [MIT License](LICENSE).
