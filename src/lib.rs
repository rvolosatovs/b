#![allow(dead_code)]

#[inline]
fn encode(it: impl IntoIterator<Item = Option<u16>>) -> impl Iterator<Item = Option<u8>> {
    it.into_iter().map(|c| match c {
        None => Some(0b00000),
        Some(2) => Some(0b00001),
        Some(4) => Some(0b00010),
        Some(8) => Some(0b00011),
        Some(16) => Some(0b00100),
        Some(32) => Some(0b00101),
        Some(64) => Some(0b00110),
        Some(128) => Some(0b00111),
        Some(256) => Some(0b01000),
        Some(512) => Some(0b01001),
        Some(1024) => Some(0b01010),
        Some(2048) => Some(0b01011),
        Some(4096) => Some(0b01100),
        Some(8192) => Some(0b01101),
        Some(16384) => Some(0b01110),
        Some(32768) => Some(0b01111),
        Some(65535) => Some(0b10000),
        _ => None,
    })
}

#[inline]
fn encode_table(table: [[Option<u16>; 4]; 4]) -> Option<u128> {
    encode(table.into_iter().flatten())
        .enumerate()
        .try_fold(0, |acc, (i, c)| {
            let c = c? as u128;
            Some(acc | (c << (5 * i)))
        })
}

#[cfg(test)]
mod tests {
    use std::iter::once;

    #[test]
    fn encode() {
        assert_eq!(
            super::encode(
                once(None).chain((1..16).map(|i| 2u16.checked_pow(i).unwrap()).map(Some))
            )
            .collect::<Option<_>>(),
            Some((0..16).collect::<Vec<_>>())
        );
    }

    #[test]
    fn encode_table() {
        assert_eq!(
            super::encode_table([
                [None, Some(2), Some(4), Some(8)],
                [Some(16), Some(32), Some(64), Some(128)],
                [Some(256), Some(512), Some(1024), Some(2048)],
                [Some(4096), Some(8192), Some(16384), Some(32768)],
        ]),
            Some(0b01111_01110_01101_01100_01011_01010_01001_01000_00111_00110_00101_00100_00011_00010_00001_00000),
        );

        assert_eq!(
            super::encode_table([
                [Some(2), Some(4), Some(8), Some(16)],
                [Some(32), Some(64), Some(128), Some(256)],
                [Some(512), Some(1024), Some(2048), Some(4096)],
                [Some(8192), Some(16384), Some(32768), Some(65535)],
        ]),
            Some(0b10000_01111_01110_01101_01100_01011_01010_01001_01000_00111_00110_00101_00100_00011_00010_00001),
        );
    }
}
