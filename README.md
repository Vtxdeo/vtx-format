# vtx-format

[![Crates.io](https://img.shields.io/crates/v/vtx-format.svg)](https://crates.io/crates/vtx-format)
[![Documentation](https://docs.rs/vtx-format/badge.svg)](https://docs.rs/vtx-format)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**vtx-format** defines the proprietary binary container format (`.vtx`) used by the VTX plugin system.

It is responsible for encapsulating standard WebAssembly Components (`.wasm`) into VTX plugin packages equipped with a Magic Header and version control. This ensures that the runtime loads only valid and version-compatible plugins.

## Specification

Currently, only the **V1** format is supported. All multibyte integers are stored in **Big-Endian** (Note: The current version only involves a single-byte version number, so endianness handling is not yet required).

### V1 Structure Layout

The file header is fixed at 4 bytes, immediately followed by the raw Wasm Component byte stream.

| Offset | Length (Bytes) | Field | Value / Description |
| :--- | :--- | :--- | :--- |
| **0x00** | 3 | **Magic Prefix** | `0x56 0x54 0x58` (ASCII: "VTX") |
| **0x03** | 1 | **Version** | `0x01` (Current Version) |
| **0x04** | N | **Payload** | Raw WebAssembly Component binary data |

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
vtx-format = "0.1"

```

## Usage

### Encoding

Encapsulate raw Wasm bytes into the `.vtx` format:

```rust
use vtx_format::encode_v1;

fn main() {
    let wasm_bytes = std::fs::read("plugin.wasm").unwrap();
    
    // Automatically adds the "VTX\x01" header
    let vtx_data = encode_v1(&wasm_bytes);
    
    std::fs::write("plugin.vtx", vtx_data).unwrap();
}

```

### Decoding

Parse a `.vtx` file and verify the header:

```rust
use vtx_format::{decode, VtxFormatError};

fn main() -> Result<(), VtxFormatError> {
    let data = std::fs::read("plugin.vtx").unwrap();

    // Returns (version, raw payload slice)
    let (version, payload) = decode(&data)?;

    println!("Detected VTX version: {}", version);
    println!("Payload size: {} bytes", payload.len());
    
    Ok(())
}

```

## Error Handling

The library provides the `VtxFormatError` enum to handle parsing errors:

* `TooShort`: Data length is less than 4 bytes.
* `InvalidPrefix`: Header is not "VTX".
* `UnsupportedVersion(u8)`: Encountered an unknown version number.

## License

MIT License
