# Shotshell

Shotshell is a random shell written in Rust. It provides a simple yet functional command-line interface (CLI) for interacting with the operating system.

## Features

- Supports basic shell commands like `cd`, `exit`, and custom command execution.
- Displays the current working directory.
- Handles command errors gracefully.

## Installation

To use Shotshell, you'll need to have Rust installed on your system. You can download Rust from the [official website]{https://www.rust-lang.org/tools/install}.

Once you have Rust set up, you can clone the repository and build the project:

```bash
git clone https://github.com/purinsu14/shotshell.git
cd shotshell
make install
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

```bash
make uninstall
make clean
```

This will remove the Shotshell executable from your system.

## Contributing

Contributions to Shotshell are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. If you'd like to contribute code, feel free to submit a pull request.

When contributing, please follow these guidelines:

1. Fork the repository and create a new branch for your changes.
2. Write clear and concise commit messages.
3. Ensure your code follows the project's coding style and conventions.
4. Test your changes thoroughly before submitting a pull request.
5. Update the documentation if necessary.

## License

Shotshell is licensed under the [MIT License](LICENSE).
