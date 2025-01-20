# OpenQA Job Filter

A very small program that translates the output of `openqa-clone-job` to something suitable as input for `openqa-mon`

## Prerequisites

- Rust toolchain (1.70 or newer)
- Cargo (included with Rust)

## Building 

```sh
cargo build --release
```

## Installation

To install the OpenQA Job Filter, run the following command:

```sh
cargo install --path .
```

## Usage

To use the OpenQA Job Filter, run the following command:

```sh
openqa-clone-job 123456 | oqa-jobfilter
```

