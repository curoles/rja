
//#![feature(test)]

//extern crate test;


pub enum ByteSize {
    KiB,
    MiB,
    GiB
}

/// const array of (factor,name) for units of information.
///
/// # Examples
///
/// ```
/// const KIB : (u64, &str) = measure_unit::BYTE_SIZE[0];
/// println!("{} = {}", KIB.1,  KIB.0);
/// ```
pub
const BYTE_SIZE: [(u64, &str); 6] = {
    let mut i = 0;
    [
        (1 << (10 * { i += 1; i }), "KiB"),
        (1 << (10 * { i += 1; i }), "MiB"),
        (1 << (10 * { i += 1; i }), "GiB"),
        (1 << (10 * { i += 1; i }), "TiB"),
        (1 << (10 * { i += 1; i }), "PiB"),
        (1 << (10 * { i += 1; i }), "EiB"),
    ]
};

/// ```
/// use measure_unit::byte_size;
/// use measure_unit::ByteSize;
/// println!("{} = {}", byte_size(ByteSize::KiB).1, byte_size(ByteSize::KiB).0);
/// println!("{} = {}", byte_size(ByteSize::MiB).1, byte_size(ByteSize::MiB).0);
/// println!("{} = {}", byte_size(ByteSize::GiB).1, byte_size(ByteSize::GiB).0);
/// ```
pub const fn byte_size(sz : ByteSize) -> (u64, &'static str)
{
    match sz {
        ByteSize::KiB => BYTE_SIZE[0],
        ByteSize::MiB => BYTE_SIZE[1],
        ByteSize::GiB => BYTE_SIZE[2],
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

    #[test]
    fn test_byte_size() {
        assert_eq!(byte_size(ByteSize::KiB).0, 1024);
        assert_eq!(byte_size(ByteSize::MiB).0, 1024*1024);
        assert_eq!(byte_size(ByteSize::GiB).0, 1024*1024*1024);
    }

    //#[bench]
    //fn benchmark_byte_size(b: &mut Bencher) {
    //    b.iter(|| {
    //        byte_size(ByteSize::GiB).0;
    //    })
    //}
}
