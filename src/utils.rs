/// Convert special character to escape sequence.
pub fn replace_escape(str: &str) -> String {
    let mut result = String::new();
    str.chars().for_each(|ch| {
        match ch {
            '\\' => result.push_str("\\\\"),
            '\"' => result.push_str("\\\""),
            '\x0C' => result.push_str("\\f"),
            '\t' => result.push_str("\\t"),
            '\n' => result.push_str("\\n"),
            '\x08' => result.push_str("\\b"),
            '\r' => result.push_str("\\r"),
            ch => result.push(ch),
        };
    });
    result
}

pub const CONT_MASK: u8 = 0b0011_1111;

#[inline]
pub(crate) const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

#[inline]
pub const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}
