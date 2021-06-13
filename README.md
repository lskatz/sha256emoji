# sha256emoji

Runs sha256sum on a file and then interprets the hashsum as an emoji.
Contributions welcome.  Immediate wishlist: unit testing; hashsum checking

## Quick start

    $ touch file.txt # zero byte
    $ sha256emoji file.txt 
    file.txt	👦🏻

## Usage

    $ sha256emoji --help
    sha256emoji 0.1.0

    USAGE:
        sha256emoji [path-to-file]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <path-to-file>    Path to the filename

## Installation

This project is written in rust and so you can use `cargo`
to install.

    git clone https://github.com/lskatz/sha256emoji
    cargo build --release ./sha256emoji
    cp -v ./sha256emoji ~/bin
