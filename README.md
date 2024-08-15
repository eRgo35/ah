# ah

A declarative package manager for Arch Linux

## What is ah?

Arch Helper is a declarative package management tool for Arch Linux. It leverages paru and topgrade for package management.

It is currently in early development phase so watch out for potential bugs!

## Installation

Check out the [releases](https://github.com/eRgo35/ah/releases) tab for the latest release!

## Building

If you want to build from source, you can do so by following the instructions below.

Install Rust :crab:

```sh
$ sudo pacman -S rustup
```

Initialize default stable

```sh
$ rustup default stable
```

Clone this repo

```sh
$ git clone https://github.com/eRgo35/ah
```

Change directory

```sh
$ cd ah
```

Build with cargo

```sh
$ cargo build --release
```

## Usage

```txt
$ ah --help
Arch Helper is a declarative package management tool for Arch Linux. It leverages paru or other package managers for seamless integration.

Usage: ah [COMMAND]

Commands:
  install         Install packages
  upgrade         Upgrade packages
  sync            Synchronize packages
  remove          Remove packages
  find            Find packages
  choose-install  Find and install packages
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
