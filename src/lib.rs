//! vtx-format：统一定义 .vtx 包格式（编码/解码）
//!
//! v1 格式：
//! - Header: 4 bytes = b"VTX\x01"
//! - Payload: component bytes (WebAssembly Component)

use thiserror::Error;

pub const VTX_PREFIX: [u8; 3] = [0x56, 0x54, 0x58]; // "VTX"
pub const VTX_VERSION_V1: u8 = 0x01;
pub const VTX_MAGIC_V1: [u8; 4] = [VTX_PREFIX[0], VTX_PREFIX[1], VTX_PREFIX[2], VTX_VERSION_V1];

#[derive(Debug, Error)]
pub enum VtxFormatError {
    #[error("vtx file too short")]
    TooShort,

    #[error("invalid vtx prefix (expected 'VTX')")]
    InvalidPrefix,

    #[error("unsupported vtx version: {0}")]
    UnsupportedVersion(u8),
}

/// 编码 v1：VTX_MAGIC_V1 + component bytes
pub fn encode_v1(component_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(VTX_MAGIC_V1.len() + component_bytes.len());
    out.extend_from_slice(&VTX_MAGIC_V1);
    out.extend_from_slice(component_bytes);
    out
}

/// 解码：返回 (version, component_bytes_slice)
pub fn decode(bytes: &[u8]) -> Result<(u8, &[u8]), VtxFormatError> {
    if bytes.len() < 4 {
        return Err(VtxFormatError::TooShort);
    }
    if bytes[0..3] != VTX_PREFIX {
        return Err(VtxFormatError::InvalidPrefix);
    }

    let version = bytes[3];
    match version {
        VTX_VERSION_V1 => Ok((version, &bytes[4..])),
        other => Err(VtxFormatError::UnsupportedVersion(other)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_v1_structure() {
        let payload = b"wasm-magic";
        let encoded = encode_v1(payload);

        // 验证头部魔数
        assert_eq!(&encoded[0..4], &VTX_MAGIC_V1);
        // 验证负载内容
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
    fn test_decode_too_short() {
        let data = b"VTX"; // 只有 3 字节
        let result = decode(data);
        assert!(matches!(result, Err(VtxFormatError::TooShort)));
    }

    #[test]
    fn test_decode_invalid_prefix() {
        // 错误的头部：VTY\x01
        let data = b"VTY\x01payload";
        let result = decode(data);
        assert!(matches!(result, Err(VtxFormatError::InvalidPrefix)));
    }

    #[test]
    fn test_decode_unsupported_version() {
        // 版本号为 0x02
        let data = b"VTX\x02payload";
        let result = decode(data);

        if let Err(VtxFormatError::UnsupportedVersion(v)) = result {
            assert_eq!(v, 2);
        } else {
            panic!("Should return UnsupportedVersion error");
        }
    }
}