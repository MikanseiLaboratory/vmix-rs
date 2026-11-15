# vmix-rs

A Rust library for interacting with vMix via TCP and HTTP APIs.

[![Crates.io](https://img.shields.io/crates/v/vmix-rs.svg)](https://crates.io/crates/vmix-rs)
[![Documentation](https://docs.rs/vmix-rs/badge.svg)](https://docs.rs/vmix-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

This library is organized into separate crates for different use cases:

- **vmix-core**: Core data structures (XML parsing optional via `xml` feature)
- **vmix-tcp**: TCP API client
- **vmix-http**: HTTP API client (async)
- **vmix-shortcuts**: Embedded vMix shortcut function catalog
- **vmix-rs**: Convenience wrapper (this crate)

## Installation

### Desktop Applications

```toml
[dependencies]
# Both TCP and HTTP support
vmix-rs = { version = "0.2.3", features = ["full"] }

# TCP only
vmix-rs = { version = "0.2.3", features = ["tcp"] }

# HTTP only
vmix-rs = { version = "0.2.3", features = ["http"] }
```

### WebAssembly

```toml
[dependencies]
# With XML parsing support
vmix-core = { version = "0.2.3", features = ["xml"] }
```

### Embedded Systems (no_std)

```toml
[dependencies]
# Struct definitions only (lightweight)
vmix-core = "0.2.3"

# With XML parsing (if needed)
vmix-core = { version = "0.2.3", features = ["xml"] }
```

## Usage

### Desktop Applications

```rust
use vmix_rs::{VmixApi, HttpVmixClient};
use std::time::Duration;

// TCP API
let client = VmixApi::new("127.0.0.1:8099".parse()?, Duration::from_secs(5))?;

// HTTP API
let http_client = HttpVmixClient::new("127.0.0.1:8088".parse()?, Duration::from_secs(5));
```

### WebAssembly

```rust
use vmix_core::{Vmix, from_str};

// Fetch XML from vMix via your HTTP client
// let xml = fetch_xml_from_vmix().await?;

// Parse XML to strongly-typed structures
let vmix_state: Vmix = from_str(&xml)?;
println!("Active input: {}", vmix_state.active);
```

### Embedded Systems (Embassy, etc.)

```rust
#![no_std]
extern crate alloc;

use vmix_core::Vmix;

// Option 1: Use struct definitions only
// Manually populate structs from TCP XMLTEXT commands
// Example: XMLTEXT vmix/active -> "1"

// Option 2: With XML parsing (requires 'xml' feature)
#[cfg(feature = "xml")]
use vmix_core::from_str;

#[cfg(feature = "xml")]
fn parse_vmix_xml(xml: &str) -> Result<Vmix, quick_xml::DeError> {
    from_str(xml)
}
```

## Examples

```bash
# TCP client
cargo run --example cli --features tcp

# HTTP client
cargo run --example http_example --features http

# TCP/HTTP comparison
cargo run --example tcp_http_comparison --features full
```

## License

MIT

## Author

Shugo Kawamura ([@FlowingSPDG](https://github.com/FlowingSPDG))
