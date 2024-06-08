[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

# Backlog

Backlog is a project management tool to track projects, boards, issues, workflows.

## Installation

Use the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install backlog.

```bash
cargo install --path .
```

## Usage

```bash
backlog --help

A command-line interface to interact with the backlog board

Usage: backlog [COMMAND]

Commands:
  init    Initialize a new backlog board
  create  Create a ticket on the board
  list    List all tickets on the board
  update  Update a ticket on the board
  delete  Delete a ticket from the board
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/) or [APACHE 2.0](https://choosealicense.com/licenses/apache-2.0/)
