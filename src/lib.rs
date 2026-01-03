//! vtx-format：统一定义 `.vtx` 包格式（编码/解码）。
//! ## v1
//! - Header: 4 bytes = `b"VTX\x01"`
//! - Payload: WebAssembly Component bytes
//!
//! ## v2
//! - Header: 4 bytes = `b"VTX\x02"`
//! - Metadata length: 4 bytes (u32 little-endian)
//! - Metadata: UTF-8 JSON bytes (length = metadata length)
//! - Payload: WebAssembly Component bytes

mod constants;
mod decode;
mod encode;
mod error;
mod types;

pub use constants::{VTX_MAGIC_V1, VTX_MAGIC_V2, VTX_PREFIX, VTX_VERSION_V1, VTX_VERSION_V2};
pub use decode::{decode, decode_with_metadata};
pub use encode::{encode_v1, encode_v2};
pub use error::VtxFormatError;
pub use types::{DecodeWithMetadataResult, DecodedVtx};

#[cfg(test)]
mod tests;
