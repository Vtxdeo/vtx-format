use thiserror::Error;

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
