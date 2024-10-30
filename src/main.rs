use core::mem::size_of;
use zerocopy::FromBytes;

fn main() {
    let q = [0x1, 0x1, 0x1, 0x1, 0x2, 0x2, 0x2, 0x2, 0xA];
    let count = q.len() / size_of::<u32>();
    let (word_buf, _suffix) = <[u32]>::ref_from_prefix_with_elems(&q, count).unwrap();

    let mut p = [0x1, 0x1, 0x1, 0x1, 0x2, 0x2, 0x2, 0x2, 0xA];
    let p_count = p.len() / size_of::<u32>();
    let (mut writeable_word_buf, _suffix) = <[u32]>::mut_from_prefix_with_elems(&mut p, p_count).unwrap();
}
