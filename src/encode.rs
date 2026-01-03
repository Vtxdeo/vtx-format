use crate::{VTX_MAGIC_V1, VTX_MAGIC_V2};

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
