mod measure_unit;
//use measure_unit::BYTE_SIZE;


fn main() {
    println!("Hello, world!");
    const KIB : (u64, &str) = measure_unit::BYTE_SIZE[0];
    println!("{} = {}", KIB.1,  KIB.0);

    use measure_unit::byte_size;
    use measure_unit::ByteSize;
    println!("{} = {}", byte_size(ByteSize::KiB).1, byte_size(ByteSize::KiB).0);
    println!("{} = {}", byte_size(ByteSize::MiB).1, byte_size(ByteSize::MiB).0);
    println!("{} = {}", byte_size(ByteSize::GiB).1, byte_size(ByteSize::GiB).0);
}
