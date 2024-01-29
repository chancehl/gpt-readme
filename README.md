# gpt-readme

A program to generate a README for your project with ChatGPT.

The project is a Rust-based software that uses the OpenAI API to automatically generate README files based on a user-provided git diff. It uses a number of Rust libraries - notably Clap, Serde, reqwest, Dotenv and Tokio.

## Pre-requisites

Before you can run the project, you need:

- Rust programming language and Cargo management tool installed on your machine. Installation instructions can be found at the [official installation guide](https://www.rust-lang.org/tools/install).
- An OpenAI API key. Learn how to get it [here](https://beta.openai.com/docs/developer-quickstart/).

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
```

2. Create a `.env` file in the project's root directory and set your OpenAI API key:

```bash
OPENAI_API_KEY=<your-api-key>
```

## Build Instructions

You can build the project using Rust's build tool, Cargo:

```bash
cargo build --release
```

## Usage

You can run the project providing the path to your project and the output file path:

```bash
cargo run --release -- --path <project-path> --out <output-file-path>
```

## Example

Here is an example of how to generate a README:

```bash
cargo run --release -- --path /path/to/project --out /path/to/output
```

## License

This project is licensed under the MIT License.

## Additional References

To better understand the libraries used if you wish to contribute:

- [Clap](https://docs.rs/clap/2.33.3/clap/) for command-line argument parsing
- [Dotenv](https://docs.rs/dotenv/0.15.0/dotenv/) for environment variable management
- [OpenAI](https://beta.openai.com/) for API details
- [Reqwest](https://docs.rs/reqwest/0.10.8/reqwest/) for HTTP client
- [Serde](https://serde.rs/) for serialization & deserialization
- [Tokio](https://tokio.rs/) for asynchronous operation

---

Shamelessly generated with LLM labor 🦾🤖
