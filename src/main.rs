use zerocopy::{FromBytes, IntoBytes, Immutable,  KnownLayout};

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
struct Foo {
    x: u32,
}

fn main() {
    let q = [0x1, 0x1, 0x1, 0x1];
    let y = Foo::ref_from_bytes(&q).unwrap();
    y.x;

    let mut p = [0x1, 0x1, 0x1, 0x1];
    let z = Foo::mut_from_bytes(&mut p).unwrap();
    z.x = 3;
}
