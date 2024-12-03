#![allow(dead_code)]

use std::{
    borrow::Cow,
    io::{BufReader, Cursor},
};

use anyhow::Ok;

pub mod aoc2024;

#[derive(rust_embed::Embed)]
#[folder = "$CARGO_MANIFEST_DIR/resources"]
pub(crate) struct Asset;

pub(crate) fn read_input(file_path: &str) -> anyhow::Result<BufReader<Cursor<Cow<'static, [u8]>>>> {
    if let Some(asset) = Asset::get(file_path) {
        let cursor = Cursor::new(asset.data);
        let reader = BufReader::new(cursor);

        return Ok(reader);
    }

    anyhow::bail!("no asset")
}

#[cfg(test)]
mod test {
    use crate::read_input;

    #[test]
    fn test_read() {
        let reader = read_input("2024/1.txt").unwrap();
        assert!(reader.capacity() != 0);
    }
}
