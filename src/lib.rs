extern crate num_bigint;

use std::io;
use std::io::prelude::*;
use std::cell::Cell;
use num_bigint::BigUint;

pub fn decode<R: Read>(stream: R) -> io::Result<BigUint> {
    let mut out = BigUint::from(0u8);
    let mut shift = 0;
    let should_continue = Cell::new(true);

    for i in stream.bytes().take_while(|_| should_continue.get()) {
        let i = i?;

        should_continue.set(i & 0x80 != 0);

        out = out | (BigUint::from(i & 0x7F) << shift);
        shift += 7;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::decode;
    use num_bigint::BigUint;

    #[test]
    fn can_decode_basic_uint() {
        assert_eq!(
            BigUint::from(300u16),
            decode(&[0b10101100, 0b00000010][..]).unwrap()
        );
    }
}
