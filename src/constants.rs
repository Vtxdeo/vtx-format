pub const VTX_PREFIX: [u8; 3] = [0x56, 0x54, 0x58]; // "VTX"
pub const VTX_VERSION_V1: u8 = 0x01;
pub const VTX_VERSION_V2: u8 = 0x02;

pub const VTX_MAGIC_V1: [u8; 4] = [VTX_PREFIX[0], VTX_PREFIX[1], VTX_PREFIX[2], VTX_VERSION_V1];
pub const VTX_MAGIC_V2: [u8; 4] = [VTX_PREFIX[0], VTX_PREFIX[1], VTX_PREFIX[2], VTX_VERSION_V2];
