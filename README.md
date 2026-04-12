# Quoter
Quoter is a CLI quote manager project written in Rust for my own learnings. However, it having some usability as a tool by others or as a base for further development, it has thus been published. It currently uses SQLite3 as the backend for storage, and thus should be quite extensible. It currently supports creation, reading, & deletion of quotes.

## Installation
1. Download the binary file from the latest release.
2. Make the binary executable (`chmod u+x quoter`)

## Usage
```
Usage: quoter [OPTIONS]

Subcommands:
  add             Add a quote
  delete  <title> Delete the quote with title <title>
  list            Display stored quote names
  read    <title> Read the quote with title <title>

Options:
    -h, --help            Display this help message
```

## Building
NOTE: This code has only been tested on GNU Linux systems. This program includes filesystem writes to user home directories (`~/.config/quoter`), and may have unusual behavior on non-GNU/Linux system.
  
_This repository requires additional libraries (sqlite3-devel) for successful linking._ Consult your distribution's documentation for necessary installation steps.

To build this project:

1. Clone this repository. `git clone https://github.com/Asher-Meachum/quoter.git`
2. Enter the directory (`cd quoter`)
3. Run `cargo build`

## Contributing 
Contributions are generally discouraged from this project. However, if you do want to contribute, please follow the branching strategy.  
`main` is for stable builds. Generally, all changes made to main should happen through `development`  
`development` is the staging branch and is where most patches should be submitted.
`documentation` is used for improving the crate documentation. Major documentation changes should be submitted here.  
Other branches may be created for specific purposes, such as error handling rewrites, adding new features, etc.