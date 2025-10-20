# HTTP Health Probe

A simple command-line tool to perform HTTP health checks on endpoints. It sends an HTTP request to a specified URL and verifies if the response status code matches the expected value.

## Features

- Supports various HTTP methods (GET, POST, etc.)
- Configurable timeout
- Customizable expected status code (defaults to 200)
- Exits with success (code 0) if the check passes, failure otherwise
- Static binary builds for easy deployment

## Usage

```bash
http-health-probe [OPTIONS] <URL>
```

### Options

- `-t, --timeout <TIMEOUT>`: Timeout in seconds for the request
- `-s, --expected-status <STATUS>`: Expected HTTP status code (default: 200)
- `-m, --method <METHOD>`: HTTP method to use (default: GET)
- `-h, --help`: Print help information

### Examples

Check if a website is up:

```bash
http-health-probe https://example.com
```

Check with a custom timeout and expected status:

```bash
http-health-probe --timeout 10 --expected-status 201 https://api.example.com/endpoint
```

Use POST method:

```bash
http-health-probe --method POST https://api.example.com/webhook
```

## Installation

### From Releases

Download the pre-built binaries from the [GitHub Releases](https://github.com/calavera/http-health-probe/releases) page.

### From GitHub Container Registry

```bash
docker run --rm ghcr.io/calavera/http-health-probe https://example.com
```

### From Crates.io

```bash
cargo install --locked http-health-probe
```

### From Source

Ensure you have Rust installed, then:

```bash
cargo build --release
```

The binary will be available at `target/release/http-health-probe`.

### Using Docker

Build the image:

```bash
docker build -t http-health-probe:latest .
```

Run the probe:

```bash
docker run --rm http-health-probe https://example.com
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
