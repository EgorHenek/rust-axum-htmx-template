# Axum Template

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/axum-latest-blue.svg)](https://github.com/tokio-rs/axum)
[![Bun](https://img.shields.io/badge/bun-latest-black.svg)](https://bun.sh)
[![HTMX](https://img.shields.io/badge/htmx-latest-blue.svg)](https://htmx.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A template for building web applications using Axum framework with integrated metrics and SQLite database.

## Prerequisites

- Rust (stable)
- [Bun](https://bun.sh/) - Required for frontend development
- SQLite

## Environment Variables

| Variable      | Description                     | Default Value      |
|--------------|----------------------------------|-------------------|
| HOST         | Server host address              | 127.0.0.1        |
| PORT         | Main server port                 | 3000             |
| METRICS_PORT | Port for metrics endpoint        | 3001             |
| DB_URL       | Database connection URL          | sqlite:./db.sqlite|

## Getting Started

1. Create your own repository from this template by clicking the "Use this template" button on GitHub
2. Clone your new repository locally

3. Build and run the server (frontend will be built automatically)
```bash
cargo run
```

## Development

For development mode:

```bash
cargo run
```

## Features

- Axum web framework
- SQLite database integration
- Metrics endpoint
- Frontend support with Bun
- Environment-based configuration
- Structured logging

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
