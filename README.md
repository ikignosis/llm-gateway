# LlmGateway

**LlmGateway** is a high-performance, flexible gateway for interacting with large language models (LLMs). Built in Rust for speed and reliability, it provides a unified interface for managing and routing requests to various LLM backends.

## Getting Started

You can install Rust following the instruction in the [official website](https://www.rust-lang.org/learn/get-started).

### Build

To compile the project:

```bash
cargo build
```

### Run

To run the gateway you can use cargo:

```bash
cargo run
```

Or run the compiled binary directly:


```bash
./target/debug/llm-gateway.exe
```

### Help
To see all available options:

```bash
cargo run -- --help
```

Or if using the binary directly:
```bash
./target/debug/llm-gateway.exe --help
```


