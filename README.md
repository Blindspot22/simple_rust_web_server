# Simple Web Server

A Rust-based web server designed to handle HTTP GET requests and serve static files. This project showcases Rust’s capabilities in handling web requests and responses, concurrency, and server-side programming.

## Features

- Handle HTTP GET requests
- Serve static HTML, CSS, and JavaScript files
- Log incoming requests and responses
- Basic routing to serve different pages

## Getting Started

### Prerequisites

- Rust (installation instructions available at [rust-lang.org](https://www.rust-lang.org/))

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/Blindspot22/simple_rust_web_server.git
   cd simple_rust_web_server

2. Build the project:

```sh
cargo build
```
3. Run the project:

```sh
cargo run
```
### Usage

The server listens on http://localhost:8080 by default. It serves static files from the project root directory.

### Examples
Open your browser and navigate to http://localhost:8080 to see the index page.
To serve other static files, simply place them in the root directory and access them via http://localhost:8080/filename.

### Sample Request
```sh
curl http://localhost:8080
```
## Project Structure
simple_web_server
├── Cargo.toml
└── src
    ├── main.rs
    └── server.rs
├── index.html
