use std;
use std::fmt;
use byteorder::{BigEndian, ByteOrder};
use std::ascii::AsciiExt;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct SpotifyId(u128);

const BASE62_DIGITS: &'static [u8] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    pub fn from_base16(id: &str) -> SpotifyId {
        assert!(id.is_ascii());
        let data = id.as_bytes();

        let mut n: u128 = 0;
        for c in data {
            let d = BASE16_DIGITS.iter().position(|e| e == c).unwrap() as u8;
            n = n * (16 as u128);
            n = n + (d as u128);
        }

        SpotifyId(n)
    }

    pub fn from_base62(id: &str) -> SpotifyId {
        assert!(id.is_ascii());
        let data = id.as_bytes();

        let mut n: u128 = 0;
        for c in data {
            let d = BASE62_DIGITS.iter().position(|e| e == c).unwrap() as u8;
            n = n * (62 as u128);
            n = n + (d as u128);
        }

        SpotifyId(n)
    }

    pub fn from_raw(data: &[u8]) -> SpotifyId {
        assert_eq!(data.len(), 16);

        let n = BigEndian::read_uint128(data, 16);

        SpotifyId(n)
    }

    pub fn to_base16(&self) -> String {
        let &SpotifyId(ref n) = self;

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[31 - i] = BASE16_DIGITS[(n.wrapping_shr(4 * i as u32) & 0xF) as usize];
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_base62(&self) -> String {
        let mut id = self.0;
        let mut data = [0u8; 22];

        for i in 0..22 {
            let digit = id % (62 as u128);
            data[21 - i] = BASE62_DIGITS[digit as usize];
            id = id / (62 as u128);
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        let mut data = [0u8; 16];

        BigEndian::write_u128(&mut data[0..16], self.0);

        data
    }
}

#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn to_base16(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .concat()
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_base16())
    }
}
