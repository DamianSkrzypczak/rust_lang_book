![rustacean](img/rustacean.png) ![logo](img/rust-logo.png) ![rustacean](img/rustacean.png)
# Rust-lang book exercises 


This repository is built as storage for [rust-lang book](
https://doc.rust-lang.org/book/) exercises that
allow for running each exercise from command line.

## Features:
- Command Line Interface built using Clap with YAML config.
    - **Hello World / Hello cargo** - self descriptive.
    - **Guessing game** - find number in 1-100 range.
    - **Functions** - just uses function that calls another function which print things...
    - **Temperature converter** - convert your Fahrenheit to Celsius and vice versa! 

## Command Line Interface:
\*each subcommand that isn't mentioned below doesn't have submenu 
and is run just by typing  
```rust_lang_book [SUBOMMAND]```.

### Main command:
    
```bash
Rust Book Excercises 0.1.0
DamianSkrzypczak <damian.piotr.skrzypczak@gmail.com>
Rust book exercises with CLI interface

USAGE:
    rust_lang_book [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    functions                Check the functions
    guessing-game            Guess-the-number game
    hello                    Runs hello_world and hello_cargo
    help                     Prints this message or the help of the given subcommand(s)
    temperature-converter    Convert temperature between Celsius and Fahrenheit
```

### temperature-converter:
```bash
rust_lang_book-temperature-converter 0.1.0
DamianSkrzypczak <damian.piotr.skrzypczak@gmail.com>
Convert temperature between Celsius and Fahrenheit

USAGE:
    rust_lang_book temperature-converter [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --from-celsius <NUMBER>       Convert Celsius to Fahrenheit
    -f, --from-fahrenheit <NUMBER>    Convert Fahrenheit to Celsius
```