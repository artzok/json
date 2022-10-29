use crate::ErrorKind::{SyntaxError, EOF};
use crate::{Error, ErrorKind, JsonArray, JsonObject, JsonValue, Result};
use core::slice;

pub struct JsonTokener {
    chars: Vec<char>,
    pos: usize,
}

const BOM: &str = "\u{feff}";

impl JsonTokener {
    // 创建并解析器
    pub fn new(str: &str) -> JsonTokener {
        let str = if str.starts_with(BOM) {
            &str[BOM.len()..]
        } else {
            str
        };
        JsonTokener {
            chars: str.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_value(&mut self) -> Result<JsonValue> {
        let c = self.next_clean_internal()?;
        return match c {
            '{' => {
                let object = self.read_object()?;
                Ok(JsonValue::Object(object))
            }
            '[' => {
                let array = self.read_array()?;
                Ok(JsonValue::Array(array))
            }
            '\'' | '"' => {
                let str = self.next_string(c)?;
                Ok(JsonValue::String(str))
            }
            _ => {
                self.pos -= 1;
                self.read_literal()
            }
        };
    }

    // 跳过空白字符和注释(/**/, //, #)
    fn next_clean_internal(&mut self) -> Result<char> {
        let len = self.chars.len();
        while self.pos < len {
            let &c = self.chars.get(self.pos).unwrap();
            self.pos += 1;
            match c {
                // 跳过空白字符
                '\t' | ' ' | '\n' | '\r' => continue,

                '/' => {
                    if self.pos == len {
                        return Ok(c);
                    }
                    match *self.chars.get(self.pos).unwrap() {
                        '*' => {
                            self.pos += 1; // to next char

                            // find comment end
                            let comment_end = index_of_all(&self.chars, &['*', '/'], self.pos);

                            match comment_end {
                                // return error if not found
                                None => return Err(Error { kind: SyntaxError }),
                                Some(end) => {
                                    self.pos = end + 2;
                                    continue;
                                }
                            }
                        }
                        '/' => {
                            // to next line and continue
                            self.pos += 1;
                            self.skip_to_end_of_line();
                            continue;
                        }
                        // return '/'
                        _ => return Ok(c),
                    }
                }
                '#' => {
                    // to next line and continue
                    self.skip_to_end_of_line();
                    continue;
                }
                _ => {
                    return Ok(c);
                }
            }
        }
        Err(Error { kind: EOF })
    }

    // 跳到行尾部
    fn skip_to_end_of_line(&mut self) {
        let end_of_line = index_of_any(&self.chars, &['\r', '\n'], self.pos);
        if let Some(pos) = end_of_line {
            self.pos = pos + 1;
        } else {
            self.pos = self.chars.len();
        }
    }

    ///
    /// Reads a sequence of key/value pairs and the trailing closing brace '}' of
    /// an object. The opening brace '{' should have already been read.
    ///
    fn read_object(&mut self) -> Result<JsonObject> {
        // create return json object
        let mut json_object = JsonObject::new();

        // Peek to see if this is the empty object.
        if self.next_clean_internal()? == '}' {
            return Ok(json_object);
        } else {
            self.pos -= 1;
        }

        loop {
            if let JsonValue::String(name) = self.next_value()? {
                //  Expect the name/value separator to be either a colon ':', an
                //  equals sign '=', or an arrow "=>". The last two are bogus but we
                //  include them because that's what the original implementation did.
                let separator = self.next_clean_internal()?;
                if separator != ':' && separator != '=' {
                    return JsonTokener::syntax_error();
                }

                // if has a char '>' after ':' or '=', then ignore it.
                if self.pos < self.chars.len() && self.chars[self.pos] == '>' {
                    self.pos += 1;
                }

                let value = self.next_value()?;
                json_object.insert(name, value);

                match self.next_clean_internal()? {
                    '}' => {
                        return Ok(json_object);
                    }
                    ';' | ',' => {
                        continue;
                    }
                    _ => {
                        return JsonTokener::syntax_error();
                    }
                }
            } else {
                return JsonTokener::syntax_error();
            }
        }
    }

    // 读取数组
    fn read_array(&mut self) -> Result<JsonArray> {
        let mut json_array = JsonArray::new();

        let mut has_trailing_separator = false;

        loop {
            match self.next_clean_internal()? {
                ']' => {
                    if has_trailing_separator {
                        json_array.push(JsonValue::None);
                    }
                    return Ok(json_array);
                }
                ',' | ';' => {
                    json_array.push(JsonValue::None);
                    has_trailing_separator = true;
                    continue;
                }
                _ => {
                    self.pos -= 1;
                }
            }

            let value = self.next_value()?;
            json_array.push(value);

            match self.next_clean_internal()? {
                ']' => {
                    return Ok(json_array);
                }
                ',' | ';' => {
                    has_trailing_separator = true;
                    continue;
                }
                _ => {
                    return JsonTokener::syntax_error();
                }
            }
        }
    }

    // 读取字符串
    fn next_string(&mut self, quote: char) -> Result<String> {
        let mut builder = String::new();

        let mut start = self.pos;

        let len = self.chars.len();
        while self.pos < len {
            let ch = self.chars[self.pos];
            self.pos += 1;

            if ch == quote {
                let str: String = self.chars[start..self.pos - 1].iter().collect();
                builder.push_str(&str);
                return Ok(builder);
            }

            if ch == '\\' {
                if self.pos >= self.chars.len() {
                    return JsonTokener::syntax_error();
                }
                let str: String = self.chars[start..self.pos - 1].iter().collect();
                builder.push_str(&str);
                let escape_str = self.read_escape_character()?;
                builder.push(escape_str);
                start = self.pos;
            }
        }
        JsonTokener::syntax_error()
    }

    // 读取一个值
    fn read_literal(&mut self) -> Result<JsonValue> {
        let literal = self.next_to_internal(&"{}[]/\\:,=;# \t\x0C");
        if literal.len() <= 0 {
            return JsonTokener::syntax_error();
        }
        if literal.eq_ignore_ascii_case("null") {
            return Ok(JsonValue::None);
        }
        if literal.eq_ignore_ascii_case("true") {
            return Ok(JsonValue::Bool(true));
        }

        if literal.eq_ignore_ascii_case("false") {
            return Ok(JsonValue::Bool(false));
        }

        if literal.contains('.') {
            let value = literal.parse::<f64>()?;
            return Ok(JsonValue::Float(value));
        }

        let mut base = 10;
        let positive;

        let number = if literal.starts_with("0x") || literal.starts_with("0X") {
            base = 16;
            positive = true;
            &literal[2..]
        } else if literal.starts_with("0") && literal.len() > 1 {
            base = 8;
            positive = true;
            &literal[1..]
        } else {
            positive = !literal.starts_with('-');
            &literal[..]
        };

        if positive {
            let value = u128::from_str_radix(number, base)?;
            Ok(JsonValue::Uint(value))
        } else {
            let value = i128::from_str_radix(number, base)?;
            Ok(JsonValue::Int(value))
        }
    }

    // 读取一个转义字符
    fn read_escape_character(&mut self) -> Result<char> {
        let ch = self.chars[self.pos];
        self.pos += 1;
        return match ch {
            'u' => {
                if self.pos + 4 > self.chars.len() {
                    return JsonTokener::syntax_error();
                }
                let str: String = self.chars[self.pos..self.pos + 4].iter().collect();
                self.pos += 4;
                let u = u32::from_str_radix(&str, 16)?;
                if let Some(ch) = char::from_u32(u) {
                    Ok(ch)
                } else {
                    Err(Error {
                        kind: ErrorKind::NumberParseError,
                    })
                }
            }
            't' => Ok('\t'),
            'b' => Ok('\x08'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            'f' => Ok('\x0C'),
            _ => Ok(ch), // '\'' | '"' | '\\'
        };
    }

    // 兑取下一个字面量
    fn next_to_internal(&mut self, excluded: &str) -> String {
        let start = self.pos;
        let len = self.chars.len();
        while self.pos < len {
            let ch = self.chars[self.pos];
            if ch == '\r' || ch == '\n' || excluded.chars().any(|c| c == ch) {
                return self.chars[start..self.pos].iter().collect();
            }
            self.pos += 1;
        }
        self.chars[start..].iter().collect()
    }

    // 语法错误
    fn syntax_error<T>() -> Result<T> {
        Err(Error { kind: SyntaxError })
    }
}

///
/// 从 list 中 查找 find
/// 
#[allow(dead_code)]
pub fn index_of<T>(list: &[T], find: &T, from: usize) -> Option<usize>
where
    T: Eq,
{
    index_of_any(list, slice::from_ref(find), from)
}

///
/// 从 list 找到 finds 任何一个的第一位索引
pub fn index_of_any<T>(list: &[T], finds: &[T], from: usize) -> Option<usize>
where
    T: Eq,
{
    if list.is_empty() {
        return None;
    }

    if finds.is_empty() {
        return None;
    }

    if from > list.len() - 1 {
        return None;
    }

    for (index, item) in list[from..].iter().enumerate() {
        if finds.iter().any(|c| c == item) {
            return Some(index + from);
        }
    }
    None
}

///
/// 从 list 中连续找到 finds 所有成员的首个索引 
fn index_of_all<T>(list: &[T], finds: &[T], from: usize) -> Option<usize>
where
    T: Eq,
{
    if list.is_empty() {
        return None;
    }

    if finds.is_empty() {
        return None;
    }

    if from > list.len() - finds.len() {
        return None;
    }

    let first = &finds[0];

    for (index, item) in list[from..].iter().enumerate() {

        if item == first {
            let mut pos = 1;

            if finds[1..].iter().all(|c| {
                let o = list.get(index + pos);
                pos += 1;
                o.is_some() && o.unwrap() == c
            }) {
                return Some(index + from);
            }
        }
    }

    None
}

#[cfg(test)]
mod test {

    use std::vec;

    use super::*;

    #[test]
    fn test_index_of() {
        let list = vec![1, 2, 3, 4];
        assert_eq!(index_of(&list, &1, 0), Some(0));
        assert_eq!(index_of(&list, &1, 2), None);
        assert_eq!(index_of(&list, &2, 0), Some(1));
        assert_eq!(index_of(&list, &2, 4), None);
        assert_eq!(index_of(&list, &4, 4), None);
        assert_eq!(index_of(&list, &4, 2), Some(3));
        assert_eq!(index_of(&list, &10, 0), None);
    }

    #[test]
    fn test_index_any() {
        let list = vec!['a', '\r', '\n', 'f'];
        assert_eq!(index_of_any(&list, &['\r', '\n'], 0), Some(1));
        assert_eq!(index_of_any(&list, &['\r', '\n'], 2), Some(2));
    }

    #[test]
    fn test_index_of_all() {
        let list = vec![1, 2, 3, 4];
        assert_eq!(index_of_all(&list, &[1, 2], 0), Some(0));
        assert_eq!(index_of_all(&list, &[1, 2], 2), None);
        assert_eq!(index_of_all(&list, &[2, 3], 0), Some(1));
        assert_eq!(index_of_all(&list, &[2, 3], 4), None);
        assert_eq!(index_of_all(&list, &[4, 10], 4), None);
        assert_eq!(index_of_all(&list, &[10, 11], 0), None);
    }
}
