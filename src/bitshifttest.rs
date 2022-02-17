fn main() {
    let thing: u32 = 0b0100001100;
    println!("{:#08x}", thing);
    println!("flags: {:#018b}", thing);
    println!("thing is {}", thing)
}