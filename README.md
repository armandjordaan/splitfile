# splitfile
A simple tool to split files by line count, written in Rust

# Introduction
This is a simple tool I wrote as a simple programming exercise in Rust. It is meant to be used to split files by line count. There is also and option to skip the first line if required. Its ideal to split large CSV files into smaller ones.

### Getting Started
- Clone the software
- Build the software with `cargo build`
- Run tests using `cargo test --all`

# Usage

As simple usage example is:

```
splitfile -f test.csv -l 10 -s
```

# Features
- Specify the lines per file in the command line args
- Skip the header line if required



# Changelog
- See `CHANGELOG.md`.

# License
- MIT License. See the `LICENSE.md` file.

# Authors
- Armand Jordaan
