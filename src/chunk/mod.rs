//! Each chunk consists of four parts:
//!
//! **Length**
//!
//! A 4-byte unsigned integer giving the number of bytes in the chunk's data field.
//! The length counts only the data field, not itself, the chunk type code, or the CRC.
//! Zero is a valid length. Although encoders and decoders should treat the length as unsigned, its value must not exceed 231 bytes.
//!
//! **Chunk Type**
//!
//! A 4-byte chunk type code. For convenience in description and in examining PNG files,
//! type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and
//! a-z, or 65-90 and 97-122 decimal). However, encoders and decoders must treat the codes as
//! fixed //! binary values, not character strings. For example, it would not be correct to
//! represent the type code IDAT by the EBCDIC equivalents of those letters.
//!
//! **Chunk Data**
//!
//! The data bytes appropriate to the chunk type, if any. This field can be of zero
//! length.
//!
//! **CRC**
//!
//!A 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding bytes in the chunk,
//! including the chunk type code and chunk data fields, but not including the length field.
//! The CRC is always present, even for chunks containing no data. See [CRC algorithm](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#CRC-algorithm).
//!
//! ---
//!
//! The chunk data length can be any number of bytes up to the maximum; therefore,
//! implementors cannot assume that chunks are aligned on any boundaries larger than bytes.
//!
//! Chunks can appear in any order, subject to the restrictions placed on each chunk type.
//! (One notable restriction is that IHDR must appear first and IEND must appear last; thus
//! the IEND chunk serves as an end-of-file marker.) Multiple chunks of the same type can
//! appear, but only if specifically permitted for that type.

#[cfg(test)]
mod unit_tests;

use std::{fmt::Display, str};

use crate::chunk_type::ChunkType;
use anyhow::{anyhow, Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

///  PNG files are essentially just a list of "chunks", each containing their
/// own data. Each chunk has a type that can be represented as a 4 character
/// string. There are standard chunk types for things like image data, but
/// there's no rule that would prevent you from inserting your own chunks with
/// whatever data you want.
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: Crc<u32>,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        // http://justsolve.archiveteam.org/wiki/CRC-32
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        Chunk {
            chunk_type,
            data,
            crc,
        }
    }
    /// A 4-byte unsigned integer giving the number of bytes in the chunk's
    /// data field. The length counts only the data field, not itself, the
    /// chunk type code, or the CRC. Zero is a valid length. Although encoders
    /// and decoders should treat the length as unsigned, its value must not
    /// exceed 231 bytes.
    fn length(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
    /// A 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding
    /// bytes in the chunk, including the chunk type code and chunk data
    /// fields, but not including the length field. The CRC is always present,
    /// even for chunks containing no data.
    fn crc(&self) -> u32 {
        let evaluation_bytes: Vec<u8> =
            [self.chunk_type.bytes().to_vec(), self.data.clone()].concat();
        self.crc.checksum(&evaluation_bytes)
    }
    fn data_as_string(&self) -> Result<String> {
        match str::from_utf8(self.data()) {
            Ok(s) => Ok(s.to_string()),
            Err(err) => Err(err.into()),
        }
    }
    fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = Error;
    /// Take a byte vec and split out the chunk elements
    fn try_from(value: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
        let (start, rest) = value.split_at(8);
        let length_bytes: [u8; 4] = start[0..4].try_into()?;
        let length: usize = u32::from_be_bytes(length_bytes).try_into()?;
        let chunk_type_bytes: [u8; 4] = start[4..8].try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;
        let (data, rest) = rest.split_at(length);
        let chunk = Chunk::new(chunk_type, data.try_into()?);
        let crc_bytes: [u8; 4] = rest[0..4].try_into()?;
        let crc = u32::from_be_bytes(crc_bytes);
        if crc != chunk.crc() {
            return Err(anyhow!(
                "crc input {} didn't match calculated {}",
                crc,
                chunk.crc()
            ));
        };
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data_as_string() {
            Ok(s) => write!(f, "{s}"),
            Err(err) => write!(f, "{}", err.to_string()),
        }
    }
}
