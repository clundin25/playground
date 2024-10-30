use zerocopy::{native_endian::U32, FromBytes};

fn main() {
    let q = [0x1, 0x1, 0x1, 0x1];
    let y = U32::ref_from_bytes(&q).unwrap();
    let _ = y.get();

    let mut p = [0x1, 0x1, 0x1, 0x1];
    let z = U32::mut_from_bytes(&mut p).unwrap();
    z.set(3);
}
