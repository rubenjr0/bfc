# Brainf*ck Compiler

Brainf*ck compiler targeting 32bit Intel x86 written in Rust.

## Dependencies
Depends on `nasm` and `ld` to assemble and link the project.

It **_probably_** only runs on Linux.

## Installation
You can install `bfc` by running

```bash
$ cargo install bfc
```

## Usage

```bash
$ bfc INPUT [output]
```

The available options are:
- `-o`: Specify the name of the output files and executable
- `-q`: Quiet, don't print messages
- `-r`: Run the executable