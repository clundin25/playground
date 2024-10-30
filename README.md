This repo documents a scenario that prevents the compiler from optimizing away `zerocopy::layout::DstLayout::validate_cast_and_convert_metadata` which contains a panic.

This code will include a panic in the "release" binary:

```rust
use zerocopy::{native_endian::U32, FromBytes};

fn main() {
    let q = [0x1, 0x1, 0x1, 0x1];
    let y = U32::ref_from_bytes(&q).unwrap();
    let _ = y.get();

    let mut p = [0x1, 0x1, 0x1, 0x1];
    let z = U32::mut_from_bytes(&mut p).unwrap();
    z.set(3);
}
```

The following code does not include a panic. It does not matter which segment is commented.

```rust
use zerocopy::{native_endian::U32, FromBytes};

fn main() {
    let q = [0x1, 0x1, 0x1, 0x1];
    let y = U32::ref_from_bytes(&q).unwrap();
    let _ = y.get();

    //let mut p = [0x1, 0x1, 0x1, 0x1];
    //let z = U32::mut_from_bytes(&mut p).unwrap();
    //z.set(3);
}
```
