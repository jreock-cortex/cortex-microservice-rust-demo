# Rust Microservice Sample

## Overview

This Rust microservice is designed as a lightweight, efficient, and highly scalable service that exemplifies the best practices for developing microservices in Rust. It showcases features such as RESTful API endpoints, asynchronous execution, and integration with essential tools and frameworks within the Rust ecosystem. This service is ideal for internal development teams looking for a reference architecture or starting point for building their own microservices in Rust.

## Features

- **Asynchronous API**: Utilizes `tokio` and `async-std` for non-blocking I/O operations.
- **RESTful Endpoints**: Offers CRUD operations over HTTP, using `warp` or `actix-web`.
- **Database Integration**: Connects to SQL and NoSQL databases with `diesel` and `mongodb` crates.
- **Configuration Management**: Leverages `config` crate for environment-specific settings.
- **Logging and Monitoring**: Integrates with `tracing` and `metrics` for insightful logging and performance monitoring.
- **Error Handling**: Implements robust error handling with `thiserror` and `anyhow` crates.
- **Testing**: Includes unit and integration tests using Rust's built-in test framework.

## Getting Started

### Prerequisites

- Rust toolchain (latest stable version recommended)
- Docker (for running dependencies like databases)
- Any preferred IDE with Rust support (e.g., VSCode with Rust plugin, IntelliJ IDEA with Rust plugin)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/jreock-cortex/cortex-microservice-rust-demo.git
cd cortex-microservice-rust-demo```

2. Build the project:
```bash
cargo build
```
3. Run the service:

```bash
cargo run
```

### Usage
This section details how to interact with the microservice, including the available endpoints and how to call them. Examples with curl:

Create Resource:

```bash
curl -X POST http://localhost:8000/resource -H 'Content-Type: application/json' -d '{"name":"example","description":"A new resource"}'
```

Get Resource:

```bash
curl http://localhost:8000/resource/{id}
```

### Contributing
We welcome contributions! Please read our CONTRIBUTING.md for details on how to submit contributions, and the process for submitting pull requests to us.

### Versioning
We use SemVer for versioning. For the versions available, see the tags on this repository.

### License
This project is licensed under the MIT License - see the LICENSE file for details.

### Acknowledgments
Rust community for the comprehensive documentation and ecosystem.
Contributors who have helped shape this project.
For more information, visit our Developer Portal or contact us through Cortex IDP.
