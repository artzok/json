use crate::{Error, ErrorKind, JsonArray, JsonObject, JsonValue, Result};
use core::slice;

pub struct JsonTokener {
    chars: Vec<char>,
    pos: usize,
    len: usize,
}

const BOM: &str = "\u{feff}";

impl JsonTokener {
    fn next(&mut self) -> char {
        let ch = self.chars[self.pos];
        self.pos += 1;
        ch
    }

    fn current(&self) -> char {
        self.chars[self.pos]
    }

    fn sub_str(&self, start: usize, end: usize) -> String {
        self.chars[start..end].iter().collect()
    }

    pub fn new(str: &str) -> JsonTokener {
        // remove bom prefix
        let str = if str.starts_with(BOM) {
            &str[BOM.len()..]
        } else {
            str
        };

        let chars: Vec<char> = str.chars().collect();
        let len = chars.len();
        JsonTokener { chars, pos: 0, len }
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
        while self.pos < self.len {
            match self.next() {
                // 跳过空白字符
                '\t' | ' ' | '\n' | '\r' => continue,
                // 跳过注释
                '/' => {
                    if self.pos == self.len {
                        return Ok('/');
                    }
                    match self.current() {
                        '*' => {
                            // to next char
                            self.pos += 1;

                            // end comment of C style
                            let comment_end = index_of_all(&self.chars, &['*', '/'], self.pos);

                            match comment_end {
                                // return error if not found
                                None => {
                                    return JsonTokener::syntax_error(
                                        "can't found comment end of C style(*/)",
                                    )
                                }
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
                        _ => return Ok('/'),
                    }
                }
                '#' => {
                    // to next line and continue
                    self.skip_to_end_of_line();
                    continue;
                }
                c => {
                    return Ok(c);
                }
            }
        }
        Err(Error::new(ErrorKind::EOF, "need next char but return EOF"))
    }

    // 跳到行尾部
    fn skip_to_end_of_line(&mut self) {
        let end_of_line = index_of_any(&self.chars, &['\r', '\n'], self.pos);
        if let Some(pos) = end_of_line {
            self.pos = pos + 1;
        } else {
            self.pos = self.len;
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
                    return JsonTokener::syntax_error(
                        "after key must have : or => separator in object",
                    );
                }

                // if has a char '>' after ':' or '=', then ignore it.
                if self.pos < self.len && self.current() == '>' {
                    self.pos += 1;
                }

                // push json value
                let value = self.next_value()?;
                json_object.insert(name, value);

                match self.next_clean_internal()? {
                    '}' => {
                        // end
                        return Ok(json_object);
                    }
                    ';' | ',' => {
                        // more field
                        continue;
                    }
                    _ => {
                        return JsonTokener::syntax_error(
                            "after value only '}' ';' ',' allow in object",
                        );
                    }
                }
            } else {
                return JsonTokener::syntax_error("must has a key for non-empty object");
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
                    return JsonTokener::syntax_error(
                        "after value only ']' ',' ';' allow in array",
                    );
                }
            }
        }
    }

    // 读取字符串
    fn next_string(&mut self, quote: char) -> Result<String> {
        let mut builder = String::new();

        let mut start = self.pos;

        while self.pos < self.len {
            let ch = self.next();
            if ch == quote {
                builder.push_str(&self.sub_str(start, self.pos - 1));
                return Ok(builder);
            }

            if ch == '\r' || ch == '\n' {
                return Err(Error::new(
                    ErrorKind::SyntaxError,
                    "string can't contain \r or \n",
                ));
            }

            if ch == '\\' {
                if self.pos >= self.len {
                    return Err(Error::new(
                        ErrorKind::EOF,
                        "ready to read escape character in string but get EOF",
                    ));
                }
                builder.push_str(&self.sub_str(start, self.pos - 1));
                let escape_str = self.read_escape_character()?;
                builder.push(escape_str);
                start = self.pos;
            }
        }
        return Err(Error::new(
            ErrorKind::EOF,
            "read string expect quote but get EOF",
        ));
    }

    // 读取一个值
    fn read_literal(&mut self) -> Result<JsonValue> {
        let literal = self.next_to_internal(&"{}[]/\\:,=;# \t\x0C");
        if literal.len() <= 0 {
            return JsonTokener::syntax_error("read a literal but get empty");
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

        let positive;
        let mut base = 10;

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
        return match self.next() {
            'u' => {
                // "\ud834"
                // return error if get eof
                if self.pos + 4 > self.len {
                    return Err(Error::new(ErrorKind::EOF, "read unicode get EOF"));
                }

                // get escape unicode string
                let unicode_str = self.sub_str(self.pos, self.pos + 4);
                self.pos += 4;

                // convert escape unicode string to unicode char
                let mut unicode = u32::from_str_radix(&unicode_str, 16)?;

                if unicode >= 0xD800 {
                    // check next code escape prefix \u
                    if self.next() != '\\' || self.next() != 'u' {
                        return Err(Error::new(
                            ErrorKind::SyntaxError,
                            "fail read next unicode, beacuse not found \\u",
                        ));
                    }
                    // check eof
                    if self.pos + 4 > self.len {
                        return Err(Error::new(ErrorKind::EOF, "read next unicode code get EOF"));
                    }

                    // get sub code str
                    let sub_unicode_str = self.sub_str(self.pos, self.pos + 4);
                    self.pos += 4;
                    let sub_unicode = u32::from_str_radix(&sub_unicode_str, 16)?;
                    unicode = (((unicode - 0xD800) << 10) | (sub_unicode - 0xDC00)) + 0x10000
                }

                if let Some(ch) = char::from_u32(unicode) {
                    Ok(ch)
                } else {
                    Err(Error::new(
                        ErrorKind::CastError,
                        "convert escape unicode string to unicode char failed",
                    ))
                }
            }
            't' => Ok('\t'),
            'b' => Ok('\x08'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            'f' => Ok('\x0C'),
            '\'' => Ok('\''),
            '\"' => Ok('\"'),
            '\\' => Ok('\\'),
            '/' => Ok('/'),
            _ => Err(Error::new(ErrorKind::SyntaxError, "error escape")),
        };
    }

    // 取下一个字面量
    fn next_to_internal(&mut self, excluded: &str) -> String {
        let start = self.pos;

        while self.pos < self.len {
            let ch = self.current();
            if ch == '\r' || ch == '\n' || excluded.chars().any(|c| c == ch) {
                return self.sub_str(start, self.pos);
            }
            self.pos += 1;
        }
        self.sub_str(start, self.len)
    }

    // 语法错误
    fn syntax_error<T>(msg: &'static str) -> Result<T> {
        Err(Error::new(ErrorKind::SyntaxError, msg))
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
