mod blocksdata;

fn main() {
    let xml = crate::blocksdata::load_blocksdata_xml();
    dbg!(xml.Blocks);
}