use crate::{
    DecodeWithMetadataResult, DecodedVtx, VtxFormatError, VTX_PREFIX, VTX_VERSION_V1,
    VTX_VERSION_V2,
};

/// 解码：返回 (version, component_bytes_slice)，对 v2 会跳过 metadata。
pub fn decode(bytes: &[u8]) -> Result<(u8, &[u8]), VtxFormatError> {
    let decoded = decode_with_metadata(bytes)?;
    Ok((decoded.version, decoded.component))
}

/// 解码并返回 metadata（仅 v2）；v1 的 metadata 为 None。
pub fn decode_with_metadata(bytes: &[u8]) -> DecodeWithMetadataResult<'_> {
    if bytes.len() < 4 {
        return Err(VtxFormatError::TooShort);
    }
    if bytes[0..3] != VTX_PREFIX {
        return Err(VtxFormatError::InvalidPrefix);
    }

    let version = bytes[3];
    match version {
        VTX_VERSION_V1 => Ok(DecodedVtx {
            version,
            metadata: None,
            component: &bytes[4..],
        }),
        VTX_VERSION_V2 => {
            let (meta, component) = decode_v2_parts(bytes)?;
            Ok(DecodedVtx {
                version,
                metadata: Some(meta),
                component,
            })
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
