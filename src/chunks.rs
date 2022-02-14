use binrw::{binrw, ReadOptions, Endian}; // #[binrw] attribute
use binrw::{BinReaderExt, BinWrite, io::Cursor}; // reading/writing utilities
use std::fs::File;
use std::io::{Seek, SeekFrom};
use opensc::chunks32h::*;

const FIRST_CHUNK_LOCATION: u32 = 786444;

//#[derive(BinRead)]
// #[binrw]
// #[br(magic = b"DOG", assert(name.len() != 0))]
// struct Dog {
//     bone_pile_count: u8,

//     #[br(big, count = bone_pile_count)]
//     bone_piles: Vec<u16>,

//     #[br(align_before = 0xA)]
//     name: NullString
// }

// #[binrw]
// struct Directory {
//     #[br(little, count = 65536)]
//     entries: Vec<DirectoryEntry>,//[DirectoryEntry; 65535],

//     guard: i32 // UNCONFIRED CORRECTNESS
// }

// #[binrw]
// #[derive(Debug)]
// struct DirectoryEntry {
//     chunkx: i32,
//     chunkz: i32,
//     index: i32
// }

// #[binrw]
// struct Chunk {
//     chunk_header: u8,
//     #[br(little, count = 65536)]
//     blocks: Vec<Block>,//    65536
//     surfacepoints: [SurfacePoint; 256]
// }

// #[binrw]
// struct Block {
//     blockdata: u32
// }

// #[binrw]
// struct SurfacePoint {
//     maxheight: u8,
//     temphumidity: u8,
//     unused1: u8,
//     unused2: u8
// }

fn main() {
    let mut file = File::open("Chunks32h.dat").unwrap();
    let directory: Directory = file.read_ne().unwrap();
    let mut chunks: Vec<Chunk> = vec![];

    

    for entry in directory.entries {
        if entry.index == -1 {
            return
        }
        println!("Found entry at x: {} z: {} with index {}", entry.chunkx, entry.chunkz, entry.index);
        file.seek(SeekFrom::Start(786444 + (entry.index as u64 * 132112))).unwrap();
        let chunk: Chunk = file.read_ne().unwrap();
        chunks.push(chunk);
    }
}