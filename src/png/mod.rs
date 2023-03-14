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
        let mut display: String = "".to_owned();
        self.chunks.iter().for_each(|chunk| {
            let new_line = format!("{}\r\n", chunk.chunk_type());
            display.push_str(&new_line)
        });
        write!(f, "{display}")
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // split byte array into chunks then add chunks to png struct
        let signature: [u8; 8] = value[0..8].try_into()?;
        if signature != Png::STANDARD_HEADER {
            return Err(anyhow!("signature {:?} does not match expected", signature));
        };
        let mut index = 8;
        let mut png = Png::from_chunks(Default::default());
        loop {
            println!("index: {} value len {}", index, value.len());
            let chunk_vec: Vec<u8> = value[index..].try_into()?;
            match Chunk::try_from(&chunk_vec) {
                Ok(chunk) => {
                    index += chunk.length() as usize + 12;
                    if format!("{}", &chunk.chunk_type()) == "IEND" {
                        png.append_chunk(chunk);
                        break;
                    }
                    png.append_chunk(chunk);
                }
                Err(err) => {
                    println!("Oops {}", err);
                    break;
                }
            };
        }
        Ok(png)
    }
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    /// Create a new png struct from a collection of chunks
    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png { chunks }
    }
    /// Add a chunk to this png
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    /// Remove a chunk from this png by its stated name, if that chunk exists
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        match self
            .chunks
            .iter()
            .position(|chunk| format!("{}", chunk.chunk_type()) == chunk_type)
        {
            Some(index) => Ok(self.chunks.remove(index)),
            None => Err(anyhow!("chunk {} not found", chunk_type)),
        }
    }
    /// Get the header chunk of this png
    pub fn header(&self) -> &[u8; 8] {
        // match self.chunk_by_type("IHDR") {
        //     Some(chunk) => &chunk.as_bytes(),
        //     None => [],
        // }
        todo!()
    }
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|&chunk| format!("{}", chunk.chunk_type()) == chunk_type)
    }
    fn as_bytes(&self) -> Vec<u8> {
        // let mut bytes: Vec<u8> = ;
        self.chunks
            .iter()
            .fold(Png::STANDARD_HEADER.to_vec(), |mut b, chunk| {
                b.append(&mut chunk.as_bytes());
                b
            })
    }
}
