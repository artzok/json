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
