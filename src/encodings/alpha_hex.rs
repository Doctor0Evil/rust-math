#[derive(Debug, Clone)]
pub struct AlphaHex;

impl AlphaHex {
    pub fn encode(bytes: &[u8]) -> String {
        const ALPHABET: &[u8; 16] = b"0123456789ABCDEF";
        let mut out = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            out.push(ALPHABET[(b >> 4) as usize] as char);
            out.push(ALPHABET[(b & 0x0F) as usize] as char);
        }
        out
    }

    pub fn decode(s: &str) -> Option<Vec<u8>> {
        if s.len() % 2 != 0 {
            return None;
        }
        let bytes = s.as_bytes();
        let mut out = Vec::with_capacity(s.len() / 2);
        for i in (0..bytes.len()).step_by(2) {
            let hi = Self::val(bytes[i])?;
            let lo = Self::val(bytes[i + 1])?;
            out.push((hi << 4) | lo);
        }
        Some(out)
    }

    fn val(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'A'..=b'F' => Some(10 + c - b'A'),
            b'a'..=b'f' => Some(10 + c - b'a'),
            _ => None,
        }
    }
}
