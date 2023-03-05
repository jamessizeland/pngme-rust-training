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

use anyhow::{anyhow, Error};

pub struct Chunk {}
