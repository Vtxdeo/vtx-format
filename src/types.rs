use crate::VtxFormatError;

pub struct DecodedVtx<'a> {
    pub version: u8,
    pub metadata: Option<&'a [u8]>,
    pub component: &'a [u8],
}

pub type DecodeWithMetadataResult<'a> = Result<DecodedVtx<'a>, VtxFormatError>;
