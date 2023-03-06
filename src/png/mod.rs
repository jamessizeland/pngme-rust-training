#[cfg(test)]
mod unit_tests;

use std::fmt::Display;

use crate::chunk::Chunk;
use anyhow::{anyhow, Error, Result};

struct Png {
    chunks: Vec<Chunk>,
}

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png { chunks }
    }
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        todo!()
    }
    pub fn header(&self) -> &[u8; 8] {
        todo!()
    }
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        todo!()
    }
    fn as_bytes(&self) -> Vec<u8> {
        self.chunks.iter().fold(Vec::new(), |mut b, chunk| {
            b.append(&mut chunk.as_bytes());
            b
        })
    }
}
