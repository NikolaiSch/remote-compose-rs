use super::{OpCode, Operation};
use std::convert::TryInto;

/// [Header](https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/remote/remote-core/src/main/java/androidx/compose/remote/core/operations/Header.java)
pub struct Header {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub metadata: std::collections::HashMap<u16, Metadata>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Metadata {
    Int(i32),
    Float(f32),
    Long(i64),
    String(String),
}

impl Header {
    pub const MAGIC_NUMBER: u32 = 0x048C0000;

    pub const CAPABILITIES_ID: u16 = 14;
    pub const WIDTH_ID: u16 = 5;
    pub const HEIGHT_ID: u16 = 6;
    pub const ROOT_ID_ID: u16 = 9;

    pub fn read(data: &[u8]) -> Result<(Self, usize), String> {
        let mut offset = 0;

        let major = u32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read major version".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert major version".to_string())?,
        );
        offset += 4;

        let minor = u32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read minor version".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert minor version".to_string())?,
        );
        offset += 4;

        let patch = u32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read patch version".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert patch version".to_string())?,
        );
        offset += 4;

        let mut metadata = std::collections::HashMap::new();

        if major < 0x10000 {
            let width = i32::from_be_bytes(
                data.get(offset..offset + 4)
                    .ok_or("Failed to read width".to_string())?
                    .try_into()
                    .map_err(|_| "Failed to convert width".to_string())?,
            );
            offset += 4;

            let height = i32::from_be_bytes(
                data.get(offset..offset + 4)
                    .ok_or("Failed to read height".to_string())?
                    .try_into()
                    .map_err(|_| "Failed to convert height".to_string())?,
            );
            offset += 4;

            let capabilities = i64::from_be_bytes(
                data.get(offset..offset + 8)
                    .ok_or("Failed to read capabilities".to_string())?
                    .try_into()
                    .map_err(|_| "Failed to convert capabilities".to_string())?,
            );
            offset += 8;

            metadata.insert(Self::WIDTH_ID, Metadata::Int(width));
            metadata.insert(Self::HEIGHT_ID, Metadata::Int(height));
            metadata.insert(Self::CAPABILITIES_ID, Metadata::Long(capabilities));

            return Ok((
                Header {
                    major,
                    minor,
                    patch,
                    metadata,
                },
                offset,
            ));
        }

        let magic = major & 0xFFFF0000;
        let major = major & 0x0000FFFF;

        if magic != Self::MAGIC_NUMBER {
            return Err("Invalid magic number".to_string());
        }

        let metadata_count = u32::from_be_bytes(
            data.get(offset..offset + 4)
                .ok_or("Failed to read metadata count".to_string())?
                .try_into()
                .map_err(|_| "Failed to convert metadata count".to_string())?,
        ) as usize;
        offset += 4;

        for _ in 0..metadata_count {
            let tag = u16::from_be_bytes(
                data.get(offset..offset + 2)
                    .ok_or("Failed to read metadata tag".to_string())?
                    .try_into()
                    .map_err(|_| "Failed to convert metadata tag".to_string())?,
            );
            offset += 2;

            // skip itemLen
            offset += 2;

            let key = tag & 0x03FF;
            let data_type = tag >> 10;

            let value = match data_type {
                0 => {
                    let val = i32::from_be_bytes(
                        data.get(offset..offset + 4)
                            .ok_or("Failed to read int metadata value".to_string())?
                            .try_into()
                            .map_err(|_| "Failed to convert int metadata value".to_string())?,
                    );
                    offset += 4;
                    Metadata::Int(val)
                }
                1 => {
                    let val = f32::from_be_bytes(
                        data.get(offset..offset + 4)
                            .ok_or("Failed to read float metadata value".to_string())?
                            .try_into()
                            .map_err(|_| "Failed to convert float metadata value".to_string())?,
                    );
                    offset += 4;
                    Metadata::Float(val)
                }
                2 => {
                    let val = i64::from_be_bytes(
                        data.get(offset..offset + 8)
                            .ok_or("Failed to read long metadata value".to_string())?
                            .try_into()
                            .map_err(|_| "Failed to convert long metadata value".to_string())?,
                    );
                    offset += 8;
                    Metadata::Long(val)
                }
                3 => {
                    let size = u32::from_be_bytes(
                        data.get(offset..offset + 4)
                            .ok_or("Failed to read string metadata size".to_string())?
                            .try_into()
                            .map_err(|_| "Failed to convert string metadata size".to_string())?,
                    ) as usize;
                    offset += 4;
                    let val = String::from_utf8(
                        data.get(offset..offset + size)
                            .ok_or("Failed to read string metadata content".to_string())?
                            .to_vec(),
                    )
                    .map_err(|_| "Failed to convert string metadata content".to_string())?;
                    offset += size;
                    Metadata::String(val)
                }
                _ => return Err(format!("Unknown metadata type: {}", data_type)),
            };

            metadata.insert(key, value);
        }

        Ok((
            Header {
                major,
                minor,
                patch,
                metadata,
            },
            offset,
        ))
    }
}

impl Operation for Header {
    fn opcode(&self) -> OpCode {
        OpCode::Header
    }
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Header")
            .field("major", &self.major)
            .field("minor", &self.minor)
            .field("patch", &self.patch)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl Clone for Header {
    fn clone(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            metadata: self.metadata.clone(),
        }
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.metadata == other.metadata
    }
}
