use std::{fs::File, io::{Seek, SeekFrom}};

use binrw::{binrw, BinReaderExt};
use modular_bitfield::{bitfield, prelude::{B10, B4, B18}};

#[binrw]
#[derive(Debug)]
pub struct DirectoryEntry {
    /// Chunk position, (1 unit equals 16 blocks)
    pub chunkx: i32,

    /// Chunk position, (1 unit equals 16 blocks)
    pub chunkz: i32,

    // Index of chunk data from the start of file, in bytes
    pub index: i32
}

#[binrw]
/// The whole directory of chunks
pub struct Directory {
    /// Directory entries, unused entries filled with 0
    #[br(little, count = 65536)]
    pub entries: Vec<DirectoryEntry>,//[DirectoryEntry; 65535],

    /// Guard entry always filled with 0
    pub guard: i32 // UNCONFIRED CORRECTNESS
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            entries: vec![],
            guard: 0,
        }
    }
}

#[bitfield]
#[binrw]
pub struct Block {
    pub blocktype: B10,
    pub lightvalue: B4,
    pub blockdata: B18
}

// #[binrw]
// /// A single block
// pub struct Block {
//     ///Bits 0 to 9 (10 bits): Block type
//     ///Bits 10 to 13 (4 bits): Block light value
//     ///Bits 14 to 31 (18 bits): Block data determining state of the block
//     pub blockdata: u32
// }

// impl Block {
//     pub fn get_id(&self) -> i10 {
//         i10::new(self.blockdata)
//     }
// }

#[binrw]
/// A single surface point
pub struct SurfacePoint {
    /// Maximum height at this point (non-air blocks)
    pub maxheight: u8,

    /// 4 low bits contain temperature, 4 high bits contain humidity
    pub temphumidity: u8,

    /// Currently unused (must be zero)
    pub unused1: u8,

    /// Currently unused (must be zero)
    pub unused2: u8
}

#[binrw]
/// Chunk header
pub struct ChunkHeader {
    /// Must be 0xDEADBEEF
    magic1: i8,

    /// Must be 0xFFFFFFFF
    magic2: i8,

    /// Chunk position (1 unit equals 16 blocks)
    chunkx: i8,

    /// Chunk position (1 unit equals 16 blocks)
    chunkz: i8,
}

#[binrw]
//#[br(little, magic = 0xdeafbeef)] //DEADBEEFFFFFFFFE
/// A single chunk
pub struct Chunk {
    /// Chunk position on X axis (1 unit equals 16 blocks)
    pub chunkx: i32,

    /// Chunk position on Y axis (1 unit equals 16 blocks)
    pub chunkz: i32,

    /// Data of all blocks in chunk (16*16*256), in x/z/y order   8448
    #[br(little, count = 65536)]
    pub blocks: Vec<Block>,

    /// Data of all surface points in chunk (16*16), in x/z order
    pub surfacepoints: [SurfacePoint; 256]
}

impl Chunk {
    pub fn get_block(&self, x: u32, y: u32, z: u32) -> u16 {
        let blockindex = y + x * 256 + z * 256 * 16; // where x is 0-15, y is 0-255 and z is 0-15
        //let blockindex = y + (16 * x) + (z * (256));
        //let arraycoord = y as u16 + (CHUNK_SIZE as u16 * x as u16) + (z as u16 * (u16::pow(CHUNK_SIZE as u16, 2)));

        return self.blocks[blockindex as usize].blocktype()
    }
}

/// The whole chunks.dat file
pub struct ChunksFile {
    /// Chunks directory
    pub directory: Directory,

    /// Array of chunks, variable size (0 to 65536 max)
    pub chunks: Vec<Chunk>
}

impl ChunksFile {
    pub fn new() -> ChunksFile {
        ChunksFile {
            directory: Directory::new(),
            chunks: vec![],
        }
    }

    /// Get a chunk from file with index
    pub fn read_chunk(file: &mut File, index: u64) -> Chunk {
        file.seek(SeekFrom::Start(786444 + (index as u64 * 132112))).unwrap();
        let chunk: Chunk = file.read_ne().unwrap();
        return chunk
    }
}