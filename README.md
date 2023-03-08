# Run App

## Prerequisites

1. Install Rust

  ```shell
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

  If you have Rust already installed, ensure it's up-to-date

  ```shell
  rustup update
  ```

2. Ensure you're using dart-sass.

  If on macOS, unistall ruby-sass first. This command will do both.

  ```shell
  gem uninstall sass && brew install sass/sass/sass
  ```

  You could check what sass version you have using

  ```shell
  sass --version
  ```

3. Install Yew

  ```shell
  rustup target add wasm32-unknown-unknown && cargo install --locked trunk
  ```

For detailed instructions, please see [Yew's installation guide](https://yew.rs/docs/getting-started/introduction)

## Clone this repo

```shell
git clone git@github.com:onweru/dates.git && cd dates
```

## Preview

From your terminal, run from

```shell
trunk serve
```

## Check component code

This is a mere demo, and has one component inside `src/main.rs` file. 