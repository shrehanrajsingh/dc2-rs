# DC2-rs: Rust Bindings for Peer-to-Peer File Transfer

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Overview

**DC2-rs** is a robust Peer-to-Peer (P2P) framework designed for efficient file transfer across networks without requiring middleware infrastructure. The system employs a sophisticated chunking mechanism that divides files into 4KB segments, ensuring fault-tolerant transfers that can be seamlessly resumed from the last valid state in case of interruption.

## Architecture

The framework implements advanced networking protocols and efficient data handling techniques. For comprehensive information regarding the architectural design and protocol standards:

> Please refer to [Protocols and Standards](PROTOCOL.md) for detailed documentation on architecture and implementation standards.

## Installation

```bash
# Clone the repository
git clone https://github.com/username/dc2-rs.git
cd dc2-rs

# Build the project
cargo build
```

## Usage

### Running the Server

Initialize the server component with the following command:

```bash
cargo run -- server 8000
```

### File Transfer Operation

In a separate terminal session, initiate a file transfer:

```bash
cargo run -- client 127.0.0.1:8000 hostfile/IPC-DBus.pdf
```

## Troubleshooting

### macOS Build Issues

If you encounter build errors related to `libSystem.dylib` on macOS, please implement one of the following solutions:

#### Solution 1: Install Xcode Command Line Tools

```bash
xcode-select --install
```

#### Solution 2: Configure Path to Apple-specific Tools

Add the following to your shell configuration file:

```bash
echo 'export PATH=/usr/bin:$PATH' >> ~/.zshrc
source ~/.zshrc
```

## Contributing

We welcome contributions to enhance DC2-rs functionality and performance.

1. **Fork** the repository
2. **Clone** your fork locally
3. Create a **feature branch** (`git checkout -b feature/amazing-enhancement`)
4. **Commit** your changes (`git commit -m 'Add amazing enhancement'`)
5. **Push** to the branch (`git push origin feature/amazing-enhancement`)
6. Open a **Pull Request**

---

© 2025 dc2-rs shrehanrajsingh | [MIT License](LICENSE)