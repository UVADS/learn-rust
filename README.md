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

## Your First Rust Application

1. Create a new application using cargo:

    ```
    cargo new my-new-project
    ```

2. `cd` into the new project and take inventory of the resources created for you.

3. Edit the `src/main.rs` file and be sure to include a `main()` function (this drives everything).

4. To build+run at the same time:

    ```
    cargo run
    ```

5. To build a release:

    ```
    cargo build
    cargo build --release
    ```

## Resources

- [**Rust**](https://www.rust-lang.org/) official page.
- [The Book](https://doc.rust-lang.org/book/)
- [Examples](https://doc.rust-lang.org/rust-by-example/)
- [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

Of interest:

- [Rewrite the `ls` command in Rust](https://endler.dev/2018/ls/) - from a [series](https://endler.dev/2017/yes/) of "useless Unix tools rewritten in Rust"
