# gpt-readme

A program to generate a README for your project with ChatGPT.

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
```

2. Create a `.env` file in the project root directory and set your OpenAI API key:

```bash
OPENAI_API_KEY=<your-api-key>
```

3. Install Rust and Cargo by following the [official installation guide](https://www.rust-lang.org/tools/install).

4. Build the project:

```bash
cargo build --release
```

## Usage

Generate a README file by providing the path to your project and the output file path:

```bash
cargo run --release -- --path <project-path> --out <output-file-path>
```

## Example

```bash
cargo run --release -- --path /path/to/project --out /path/to/output
```

## License

This project is licensed under the [MIT License](LICENSE).

---

Shamelessly generated with LLM labor 🦾🤖
