# vmix-core

Core data structures for vMix API. This crate is `no_std` compatible and can be used in embedded environments and WebAssembly.

## Features

- `no_std` compatible (requires `alloc`)
- Strongly-typed data structures for vMix state
- Optional XML parsing via `xml` feature
- Zero network dependencies

## Usage

### Without XML parsing (lightweight)

```rust
use vmix_core::Vmix;

// Use struct definitions only
// XML parsing is handled by your own implementation
```

### With XML parsing

```toml
[dependencies]
vmix-core = { version = "0.2.3", features = ["xml"] }
```

```rust
use vmix_core::{Vmix, from_str};

// Parse XML from vMix
let xml = r#"<vmix><version>1.0</version>...</vmix>"#;
let vmix_state: Vmix = from_str(xml)?;

println!("vMix version: {}", vmix_state.version);
println!("Active input: {}", vmix_state.active);
```

## Features

- `xml` (optional): Enable XML parsing with `quick-xml`
- `std` (optional): Enable standard library support

## License

MIT
