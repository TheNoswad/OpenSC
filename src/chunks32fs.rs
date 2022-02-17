use binrw::binrw;

const RegionDirectoryOffset: usize = 4;
const RegionDirectoryEntrySize: usize = 8;

#[binrw]
pub struct Directory {
    #[br(count = 256)] // 256 entries in the directory
    entries: Vec<DirectoryEntry>
}

#[binrw]
pub struct DirectoryEntry {
    offset: i32,
    size: i32
}

#[binrw]
pub struct CompressedChunk {
    /// Magic Header
    magic: u32,
    data: Vec<>
}

pub struct RegionFile {
    directory: Directory,
    chunks: Vec<Chunk>
}