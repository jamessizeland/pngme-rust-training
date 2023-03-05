//! A 4-byte chunk type code. For convenience in description and in examining
//! PNG files, type codes are restricted to consist of uppercase and lowercase
//! ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal). However,
//! encoders and decoders must treat the codes as fixed binary values, not
//! character strings. For example, it would not be correct to represent the
//! type code IDAT by the EBCDIC equivalents of those letters.

#[cfg(test)]
mod unit_tests;

use anyhow::{anyhow, Error};
use std::str;

/// Chunk type codes are assigned so that a decoder can determine some
/// properties of a chunk even when it does not recognize the type code.
///
/// These rules are intended to allow safe, flexible extension of the PNG
/// format, by allowing a decoder to decide what to do when it encounters an
/// unknown chunk.
///
/// The naming rules are not normally of interest when the decoder does
/// recognize the chunk's type.
#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    raw: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    /// Check that all bytes are valid utf-8 and ASCII
    fn try_from(value: [u8; 4]) -> std::result::Result<Self, Self::Error> {
        match str::from_utf8(&value) {
            Ok(_) => Ok(ChunkType { raw: value }),
            Err(err) => Err(err.into()),
        }
    }
}
impl str::FromStr for ChunkType {
    type Err = Error;

    /// Check that string is valid ASCII and that there are at least 4 bytes
    /// to make a valid chunk type
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes: [u8; 4] = s.as_bytes().try_into()?;
        if bytes.iter().fold(true, |valid, byte| {
            (byte.is_ascii_lowercase() || byte.is_ascii_uppercase()) && valid
        }) {
            return Ok(ChunkType { raw: bytes });
        }
        Err(anyhow!("chunk type: {:?} is invalid", bytes))
    }
}
impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match str::from_utf8(&self.bytes()) {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "invalid"),
        }
    }
}
impl ChunkType {
    /// Get the raw bytes of this chunk type
    pub fn bytes(&self) -> [u8; 4] {
        self.raw
    }
    /// Check if chunk is valid ASCII and valid PNG
    pub fn is_valid(&self) -> bool {
        self.bytes().iter().fold(true, |valid, byte| {
            (byte.is_ascii_lowercase() || byte.is_ascii_uppercase()) && valid
        }) && self.is_reserved_bit_valid()
    }
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary.
    ///
    /// Chunks that are not strictly necessary in order to meaningfully
    /// display the contents of the file are known as "ancillary" chunks. A
    /// decoder encountering an unknown chunk in which the ancillary bit is 1
    /// can safely ignore the chunk and proceed to display the image. The time
    /// chunk (tIME) is an example of an ancillary chunk.
    ///
    /// Chunks that are necessary for successful display of the file's
    /// contents are called "critical" chunks. A decoder encountering an
    /// unknown chunk in which the ancillary bit is 0 must indicate to the
    /// user that the image contains information it cannot safely interpret.
    /// The image header chunk (IHDR) is an example of a critical chunk.
    pub fn is_critical(&self) -> bool {
        let byte = self.bytes()[0];
        byte >> 5 & 1 == 0
    }
    /// 0 (uppercase) = public, 1 (lowercase) = private.
    ///
    /// A public chunk is one that is part of the PNG specification or is
    /// registered in the list of PNG special-purpose public chunk types.
    /// Applications can also define private (unregistered) chunks for their
    /// own purposes. The names of private chunks must have a lowercase second
    /// letter, while public chunks will always be assigned names with
    /// uppercase second letters. Note that decoders do not need to test the
    /// private-chunk property bit, since it has no functional significance;
    /// it is simply an administrative convenience to ensure that public and
    /// private chunk names will not conflict.
    pub fn is_public(&self) -> bool {
        let byte = self.bytes()[1];
        byte >> 5 & 1 == 0
    }
    /// Must be 0 (uppercase) in files conforming to this version of PNG.
    ///
    /// The significance of the case of the third letter of the chunk name is
    /// reserved for possible future expansion. At the present time all chunk
    /// names must have uppercase third letters.
    ///
    /// (Decoders should not complain about a lowercase third letter, however,
    /// as some future version of the PNG specification could define a meaning
    /// for this bit. It is sufficient to treat a chunk with a lowercase third
    /// letter in the same way as any other unknown chunk type.)
    pub fn is_reserved_bit_valid(&self) -> bool {
        let byte = self.bytes()[2];
        byte >> 5 & 1 == 0
    }
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    ///
    /// This property bit is not of interest to pure decoders, but it is
    /// needed by PNG editors (programs that modify PNG files). This bit
    /// defines the proper handling of unrecognized chunks in a file that is
    /// being modified.
    pub fn is_safe_to_copy(&self) -> bool {
        let byte = self.bytes()[3];
        byte >> 5 & 1 == 1
    }
}
