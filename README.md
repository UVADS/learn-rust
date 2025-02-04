# learn-rust

Learn about the Rust programming language.

<img src="https://rustacean.net/assets/rustacean-flat-happy.png" aligln="right" style="float:right; max-width:25%" />

From a Data Science & Analytics Club workshop, held February 4, 2025. [**Slides**](introducing-rust.pdf).

## How to use this repository

1. Clone the repo as usual to your local workstation.
2. This repo includes submodules. To initialize (fetch) all submodules, run this:

    ```bash
    git submodule init
    git submodule update
    ```

3. To do these two steps at once, try this command:

    ```bash
    git clone --recurse-submodules git@github.com:uvads/learn-rust.git
    ```

## Setting Up

To work with Rust it is recommended that you install [**`rustup`**](https://doc.rust-lang.org/cargo/getting-started/installation.html) which includes Cargo and other utilities. Cargo does the following:

- Init new Rust projects
- Package manager
- Compiler
- Runs tests
- Publish packages/releases to https://crates.io/
