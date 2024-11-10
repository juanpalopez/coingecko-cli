# CoinGecko CLI

In this project, we will be implementing a CLI that maps to CoingGecko API
https://www.coingecko.com/en/api/documentation#

| Command         | Status      |
| --------------- | ----------- |
| ping            | Done        |
| simple          | In progress |
| coins           |             |
| contract        |             |
| asset_platforms | In progress |
| categories      |             |
| exchanges       |             |
| indexes         |             |
| derivatives     |             |
| nfts            |             |
| exchange_rates  |             |
| search          |             |
| trending        |             |
| global          |             |
| companies       |             |

Each of them will have one or more subcommands, you may find more details in the API documentation.
Initially, we will be supporting the Free API which currently supports **10-30 calls/minute**
and doesn't require an API key.

The project will be implemented using Rust.


# How to install the project

To install the project, please follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/juanpalopez/coingecko-cli.git
    ```

2. Change to the project directory:
    ```sh
    cd coingecko-cli
    ```

3. Build the project using Cargo:
    ```sh
    # build debug mode
    cargo build
    # or
    # build release mode
    cargo build --release
    ```

Congratulations! The project is now installed and ready to use.


# How to Run the CLI

After installing the project, you can run the CLI with the following steps:

1. If you're not already in the project directory, navigate to it:
    ```sh
    cd coingecko-cli
    ```

2. To run the CLI in debug mode, use:
    ```sh
    cargo run
    ```

3. To run the CLI with arguments, append them after `--` in the command line:
    ```sh
    cargo run -- [arguments]
    ```

   For example, to run a command `ping`, you would use:
    ```sh
    cargo run -- ping
    ```

4. If you built the project in release mode and want to run the release version, navigate to the `target/release` directory and run the executable directly:
    ```sh
    cd target/release
    ./coingecko-cli [arguments]
    ```

   Replace `[arguments]` with any specific commands or options your CLI supports.

5. To view all available commands and their descriptions, you can use the help command:
    ```sh
    cargo run -- --help
    ```
   Or, if running a built-release version:
    ```sh
    ./coingecko-cli --help

Congratulations! You are now running the CLI.