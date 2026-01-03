//! vtx-format：统一定义 `.vtx` 包格式（编码/解码）。
//!
//! ## v1
//! - Header: 4 bytes = `b"VTX\x01"`
//! - Payload: WebAssembly Component bytes
//!
//! ## v2
//! - Header: 4 bytes = `b"VTX\x02"`
//! - Metadata length: 4 bytes (u32 little-endian)
//! - Metadata: UTF-8 JSON bytes (length = metadata length)
//! - Payload: WebAssembly Component bytes

use thiserror::Error;

pub const VTX_PREFIX: [u8; 3] = [0x56, 0x54, 0x58]; // "VTX"
pub const VTX_VERSION_V1: u8 = 0x01;
pub const VTX_VERSION_V2: u8 = 0x02;

pub const VTX_MAGIC_V1: [u8; 4] = [VTX_PREFIX[0], VTX_PREFIX[1], VTX_PREFIX[2], VTX_VERSION_V1];
pub const VTX_MAGIC_V2: [u8; 4] = [VTX_PREFIX[0], VTX_PREFIX[1], VTX_PREFIX[2], VTX_VERSION_V2];

#[derive(Debug, Error)]
pub enum VtxFormatError {
    #[error("vtx file too short")]
    TooShort,

    #[error("invalid vtx prefix (expected 'VTX')")]
    InvalidPrefix,

    #[error("unsupported vtx version: {0}")]
    UnsupportedVersion(u8),

    #[error("invalid vtx v2 metadata length")]
    InvalidMetadataLength,
}

/// 编码 v1：VTX_MAGIC_V1 + component bytes
pub fn encode_v1(component_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(VTX_MAGIC_V1.len() + component_bytes.len());
    out.extend_from_slice(&VTX_MAGIC_V1);
    out.extend_from_slice(component_bytes);
    out
}

/// 编码 v2：VTX_MAGIC_V2 + metadata_len(u32 LE) + metadata_json + component bytes
pub fn encode_v2(component_bytes: &[u8], metadata_json: &[u8]) -> Vec<u8> {
    let len = metadata_json.len() as u32;
    let mut out =
        Vec::with_capacity(VTX_MAGIC_V2.len() + 4 + metadata_json.len() + component_bytes.len());
    out.extend_from_slice(&VTX_MAGIC_V2);
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(metadata_json);
    out.extend_from_slice(component_bytes);
    out
}

/// 解码：返回 (version, component_bytes_slice)，对 v2 会跳过 metadata。
pub fn decode(bytes: &[u8]) -> Result<(u8, &[u8]), VtxFormatError> {
    let (version, _meta, component) = decode_with_metadata(bytes)?;
    Ok((version, component))
}

/// 解码并返回 metadata（仅 v2）；v1 的 metadata 为 None。
pub fn decode_with_metadata(bytes: &[u8]) -> Result<(u8, Option<&[u8]>, &[u8]), VtxFormatError> {
    if bytes.len() < 4 {
        return Err(VtxFormatError::TooShort);
    }
    if bytes[0..3] != VTX_PREFIX {
        return Err(VtxFormatError::InvalidPrefix);
    }

    let version = bytes[3];
    match version {
        VTX_VERSION_V1 => Ok((version, None, &bytes[4..])),
        VTX_VERSION_V2 => {
            let (meta, component) = decode_v2_parts(bytes)?;
            Ok((version, Some(meta), component))
        }
        other => Err(VtxFormatError::UnsupportedVersion(other)),
    }
}

fn decode_v2_parts(bytes: &[u8]) -> Result<(&[u8], &[u8]), VtxFormatError> {
    if bytes.len() < 8 {
        return Err(VtxFormatError::TooShort);
    }
    let meta_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let meta_start = 8usize;
    let meta_end = meta_start.saturating_add(meta_len);
    if meta_end > bytes.len() {
        return Err(VtxFormatError::InvalidMetadataLength);
    }
    Ok((&bytes[meta_start..meta_end], &bytes[meta_end..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_v1_structure() {
        let payload = b"wasm-magic";
        let encoded = encode_v1(payload);
        assert_eq!(&encoded[0..4], &VTX_MAGIC_V1);
        assert_eq!(&encoded[4..], payload);
    }

    #[test]
    fn test_decode_valid_v1() {
        let payload = b"component-data";
        let mut data = Vec::new();
        data.extend_from_slice(&VTX_MAGIC_V1);
        data.extend_from_slice(payload);

        let result = decode(&data);
        assert!(result.is_ok());
        let (version, body) = result.unwrap();

        assert_eq!(version, VTX_VERSION_V1);
        assert_eq!(body, payload);
    }

    #[test]
    fn test_encode_decode_v2_with_metadata() {
        let meta = br#"{"schema":1,"author":"a","sdk_version":"0.1.8"}"#;
        let payload = b"component-data";
        let encoded = encode_v2(payload, meta);

        let (ver, meta_out, body) = decode_with_metadata(&encoded).unwrap();
        assert_eq!(ver, VTX_VERSION_V2);
        assert_eq!(meta_out.unwrap(), meta);
        assert_eq!(body, payload);

        let (ver2, body2) = decode(&encoded).unwrap();
        assert_eq!(ver2, VTX_VERSION_V2);
        assert_eq!(body2, payload);
    }

    #[test]
    fn test_decode_too_short() {
        let data = b"VTX";
        let result = decode(data);
        assert!(matches!(result, Err(VtxFormatError::TooShort)));
    }

    #[test]
    fn test_decode_invalid_prefix() {
        let data = b"VTY\x01payload";
        let result = decode(data);
        assert!(matches!(result, Err(VtxFormatError::InvalidPrefix)));
    }

    #[test]
    fn test_decode_unsupported_version() {
        let data = b"VTX\x99payload";
        let result = decode(data);
        assert!(matches!(
            result,
            Err(VtxFormatError::UnsupportedVersion(0x99))
        ));
    }

    #[test]
    fn test_decode_v2_invalid_metadata_len() {
        let mut data = Vec::new();
        data.extend_from_slice(&VTX_MAGIC_V2);
        data.extend_from_slice(&(100u32.to_le_bytes()));
        data.extend_from_slice(b"short");
        let result = decode_with_metadata(&data);
        assert!(matches!(result, Err(VtxFormatError::InvalidMetadataLength)));
    }
}
