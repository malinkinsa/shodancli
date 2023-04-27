# Shodan CLI Tool

![GitHub](https://img.shields.io/github/license/malinkinsa/shodancli)

This is a simple command-line interface (CLI) tool written in Rust, that allows users to fetch data related to IP addresses from the Shodan API.

## Requirements

    Rust programming language (version 1.51.0 or higher)
    Shodan API key

## Usage
- Download pre-built binary files from the "Releases" section. Depending on your system architecture, choose the appropriate file:

  | File name                                 | System                                              | Architecture              | Toolchain                      |
  |-------------------------------------------|-----------------------------------------------------|---------------------------|--------------------------------|
  | shodancli-aarch64-apple-darwin.tar.gz     | macOS with Apple Silicon (M1/M2) processor          | ARM64                     | N/A                            |
  | shodancli-universal-apple-darwin.tar.gz   | macOS with Intel/Silicon processor                  | Universal (x86_64, ARM64) | N/A                            |
  | shodancli-x86_64-apple-darwin.tar.gz      | macOS with Intel x86_64 processor                   | x86_64                    | N/A                            |
  | shodancli-x86_64-pc-windows-gnu.zip       | Windows with x86_64 processor, using GNU toolchain  | x86_64                    | GNU toolchain                  |
  | shodancli-x86_64-pc-windows-msvc.zip      | Windows with x86_64 processor, using MSVC toolchain | x86_64                    | Microsoft Visual C++ toolchain |
  | shodancli-x86_64-unknown-linux-gnu.tar.gz | Linux with x86_64 processor, using GNU toolchain    | x86_64                    | GNU toolchain                  |

- Set the SHODAN_API_KEY environment variable to your Shodan API key.
- Run the tool with the command cargo run -- -t <TARGET_IP>.
- You can specify multiple IP addresses separated by comma, like cargo run -- -t 127.0.0.1,8.8.8.8.

## Build and run the Shodan CLI Tool:

- Install Rust programming language: [Link](https://www.rust-lang.org/tools/install).
- Clone this repository:
  ```bash
  git clone git@github.com:malinkinsa/shodancli.git
  ```
- Set your Shodan API key as an environment variable by running the command export SHODAN_API_KEY=<your_api_key>.
- Navigate to the project directory in your terminal.
- Build the project by running the command:
  ```bash
  cargo build --release
  ```
  This will compile the Rust code and generate an executable binary file in the target/release directory.

- Run the tool by executing the binary file with the command: 
  ```bash
  ./target/release/shodancli -t <TARGET_IP> 
  ```
  where <TARGET_IP> is the IP address(es) you want to check. You can specify multiple IP addresses separated by comma, like ./target/release/shodan-cli -t 127.0.0.1,8.8.8.8


## Functionality

The tool uses the reqwest and serde libraries to fetch data from the Shodan API and deserialize it into Rust objects. It then displays the relevant information for the target IP address(es) in a formatted output. If the target is not found, displaying a message indicating no entries found.
Dependencies

The tool has the following dependencies:

    clap: used to parse command-line arguments
    reqwest: used to make HTTP requests to the Shodan API
    serde and serde_json: used to deserialize the API response into Rust objects