use crate::*;

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

    let decoded = decode_with_metadata(&encoded).unwrap();
    assert_eq!(decoded.version, VTX_VERSION_V2);
    assert_eq!(decoded.metadata.unwrap(), meta);
    assert_eq!(decoded.component, payload);

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
