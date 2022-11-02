use crate::{Error, ErrorKind, JsonArray, JsonObject, JsonValue, Result};

pub struct JsonTokener<'a> {
    bytes: &'a [u8],
    pos: usize,
    len: usize,
}

const BOM: &str = "\u{feff}";

/// Parser of json string.
impl<'a> JsonTokener<'a> {
    /// Create and init parser.
    pub fn new(str: &str) -> JsonTokener {
        // remove bom prefix
        let str = if str.starts_with(BOM) {
            &str[BOM.len()..]
        } else {
            str
        };

        let bytes = str.as_bytes();
        let len = bytes.len();
        JsonTokener { bytes, pos: 0, len }
    }

    #[inline]
    fn next(&mut self) -> u8 {
        let b = self.bytes[self.pos];
        self.pos += 1;
        b
    }

    #[inline]
    fn current(&mut self) -> u8 {
        self.bytes[self.pos]
    }

    /// `pos` plus `offset` will exceed `len - 1`.
    #[inline]
    fn will_eof(&self, offset: usize) -> bool {
        return self.pos + offset >= self.len;
    }

    /// `pos` go back to the previous position.
    #[inline]
    fn back(&mut self) {
        self.pos -= 1;
    }

    // `pos` to next one.
    #[inline]
    fn advance(&mut self, offset: usize) {
        self.pos += offset;
    }

    /// Create sub string from chars.
    #[inline]
    fn sub_str(&self, start: usize, end: usize) -> &str {
        std::str::from_utf8(&self.bytes[start..end]).unwrap()
    }

    /// parse next JSON element.
    pub fn next_value(&mut self) -> Result<JsonValue> {
        let c = self.next_clean_internal()?;
        return match c {
            b'{' => {
                let object = self.read_object()?;
                Ok(JsonValue::Object(object))
            }
            b'[' => {
                let array = self.read_array()?;
                Ok(JsonValue::Array(array))
            }
            b'\'' | b'"' => {
                let str = self.next_string(c)?;
                Ok(JsonValue::String(str))
            }
            _ => {
                self.back();
                self.read_literal()
            }
        };
    }

    #[inline]
    pub fn index_of_all(&self, bytes: &[u8]) -> Option<usize> {
        let mut index = self.pos;
        let mut offset = 0;
        let mut go_on = true;

        loop {
            if index < self.len {
                if self.bytes[index] == bytes[offset] {
                    offset += 1;
                    if offset == bytes.len() {
                        return Some(index);
                    }
                    go_on = false;
                } else if !go_on {
                    return None;
                }
            } else {
                return None;
            }
            index += 1;
        }
    }

    #[inline]
    pub fn index_of_any(&self, bytes: &[u8]) -> Option<usize> {
        let mut index = self.pos;
        loop {
            if index < self.len {
                if bytes.contains(&self.bytes[index]) {
                    return Some(index);
                }
            } else {
                return None;
            }
            index += 1;
        }
    }

    // skip comments and blank char.
    fn next_clean_internal(&mut self) -> Result<u8> {
        while self.pos < self.len {
            match self.next() {
                // blank.
                b'\n' | b'\t' | b' ' | b'\r' => continue,
                // comments.
                b'/' => {
                    if self.will_eof(0) {
                        return Ok(b'/');
                    }
                    match self.current() {
                        b'*' => {
                            // to next char
                            self.advance(1);

                            // end comment of C style
                            let comment_end = self.index_of_all(&[b'*', b'/']);

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
                        b'/' => {
                            // to next line and continue
                            self.advance(1);

                            self.skip_to_end_of_line();
                            continue;
                        }
                        // return '/'
                        _ => return Ok(b'/'),
                    }
                }
                b'#' => {
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

    fn skip_to_end_of_line(&mut self) {
        let end_of_line = self.index_of_any(&[b'\n', b'\r']);
        if let Some(pos) = end_of_line {
            self.pos = pos + 1;
        } else {
            self.pos = self.len;
        }
    }

    /// Reads a sequence of key/value pairs and the trailing closing brace '}' of
    /// an object. The opening brace '{' should have already been read.
    fn read_object(&mut self) -> Result<JsonObject> {
        // create return json object
        let mut json_object = JsonObject::new();

        // Peek to see if this is the empty object.
        if self.next_clean_internal()? == b'}' {
            return Ok(json_object);
        } else {
            self.back();
        }

        loop {
            if let JsonValue::String(name) = self.next_value()? {
                //  Expect the name/value separator to be either a colon ':', an
                //  equals sign '=', or an arrow "=>". The last two are bogus but we
                //  include them because that's what the original implementation did.
                let separator = self.next_clean_internal()?;
                if separator != b':' && separator != b'=' {
                    return JsonTokener::syntax_error(
                        "after key must have : or => separator in object",
                    );
                }

                // if has a char '>' after ':' or '=', then ignore it.
                if !self.will_eof(0) && self.current() == b'>' {
                    self.advance(1);
                }

                // push json value
                let value = self.next_value()?;
                json_object.insert(name, value);

                match self.next_clean_internal()? {
                    b'}' => {
                        // end
                        return Ok(json_object);
                    }
                    b';' | b',' => {
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

    fn read_array(&mut self) -> Result<JsonArray> {
        let mut json_array = JsonArray::new();

        let mut has_trailing_separator = false;

        loop {
            match self.next_clean_internal()? {
                b']' => {
                    if has_trailing_separator {
                        json_array.push(JsonValue::Null);
                    }
                    return Ok(json_array);
                }
                b',' | b';' => {
                    json_array.push(JsonValue::Null);
                    has_trailing_separator = true;
                    continue;
                }
                _ => {
                    self.back();
                }
            }

            let value = self.next_value()?;
            json_array.push(value);

            match self.next_clean_internal()? {
                b']' => {
                    return Ok(json_array);
                }
                b',' | b';' => {
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

    fn next_string(&mut self, quote: u8) -> Result<String> {
        let mut builder = String::with_capacity(20);

        let mut start = self.pos;

        while self.pos < self.len {
            let ch = self.next();
            if ch == quote {
                builder.push_str(self.sub_str(start, self.pos - 1));
                return Ok(builder);
            }

            if ch == b'\n' || ch == b'\r' {
                return Err(Error::new(
                    ErrorKind::SyntaxError,
                    "string can't contain \\r or \\n",
                ));
            }

            if ch == b'\\' {
                if self.will_eof(0) {
                    return Err(Error::new(
                        ErrorKind::EOF,
                        "ready to read escape character in string but get EOF",
                    ));
                }
                builder.push_str(self.sub_str(start, self.pos - 1));
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

    /// read a value, eg: true, false, null, number, etc.
    fn read_literal(&mut self) -> Result<JsonValue> {
        let literal = self.next_to_internal(&[
            b' ', b'{', b'}', b'[', b']', b'/', b'\\', b':', b',', b'=', b';', b'#', b'\t', b'\x0C',
        ]);
        if literal.len() <= 0 {
            return JsonTokener::syntax_error("read a literal but get empty");
        }
        if literal.eq_ignore_ascii_case("null") {
            return Ok(JsonValue::Null);
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

    fn read_escape_character(&mut self) -> Result<char> {
        return match self.next() {
            b'u' => {
                // return error if get eof, need 4 char
                if self.will_eof(3) {
                    return Err(Error::new(ErrorKind::EOF, "read unicode get EOF"));
                }

                // get escape unicode string
                let unicode_str = self.sub_str(self.pos, self.pos + 4);
                // convert escape unicode string to unicode char
                let mut unicode = u32::from_str_radix(unicode_str, 16)?;
                self.advance(4);

                if unicode >= 0xD800 {
                    // check next code escape prefix \u
                    if self.next() != b'\\' || self.next() != b'u' {
                        return Err(Error::new(
                            ErrorKind::SyntaxError,
                            "fail read next unicode, beacuse not found \\u",
                        ));
                    }
                    // check eof: need 4 char
                    if self.will_eof(3) {
                        return Err(Error::new(ErrorKind::EOF, "read next unicode code get EOF"));
                    }

                    // get sub code str
                    let sub_unicode_str = self.sub_str(self.pos, self.pos + 4);
                    let sub_unicode = u32::from_str_radix(sub_unicode_str, 16)?;
                    self.advance(4);
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
            b't' => Ok('\t'),
            b'b' => Ok('\x08'),
            b'n' => Ok('\n'),
            b'r' => Ok('\r'),
            b'f' => Ok('\x0C'),
            b'\'' => Ok('\''),
            b'\"' => Ok('\"'),
            b'\\' => Ok('\\'),
            b'/' => Ok('/'),
            _ => Err(Error::new(ErrorKind::SyntaxError, "error escape")),
        };
    }

    fn next_to_internal(&mut self, excluded: &[u8]) -> &str {
        let start = self.pos;

        while self.pos < self.len {
            let ch = self.current();
            if ch == b'\n' || ch == b'\r' || excluded.contains(&ch) {
                return self.sub_str(start, self.pos);
            }
            self.pos += 1;
        }
        self.sub_str(start, self.len)
    }

    fn syntax_error<T>(msg: &'static str) -> Result<T> {
        Err(Error::new(ErrorKind::SyntaxError, msg))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn tokener_bom() {
        let str = "\u{feff}\"string\"";
        let json = JsonTokener::new(str).next_value();
        assert!(json.is_ok());
        let json = json.unwrap();
        assert_eq!(json.as_str().unwrap(), "string");
    }

    #[test]
    #[should_panic(expected = "can't found comment end of C style(*/)")]
    fn tokener_comemnt_error() {
        let str = "/*";
        JsonTokener::new(str).next_value().unwrap();
    }

    #[test]
    fn tokener_comment_ok() {
        let str = "//hello\n\"string\"";
        let json = JsonTokener::new(str).next_value().unwrap();
        assert_eq!(json.as_string().unwrap(), "string");

        let str = "#hello\n\"string\"";
        let json = JsonTokener::new(str).next_value().unwrap();
        assert_eq!(json.as_string().unwrap(), "string");

        let str = "/*hello*/\n\"string\"";
        let json = JsonTokener::new(str).next_value().unwrap();
        assert_eq!(json.as_string().unwrap(), "string");
    }

    #[test]
    #[should_panic(expected = "read a literal but get empty")]
    fn tokener_comment_error() {
        let str = "/";
        JsonTokener::new(str).next_value().unwrap();
    }

    #[test]
    #[should_panic(expected = "need next char but return EOF")]
    fn tokener_comment_error2() {
        let str = "//";
        JsonTokener::new(str).next_value().unwrap();
    }

    #[test]
    fn tokener_separator() {
        let str = "{\"key\":1}";
        assert!(JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_object()
            .is_ok());

        let str = "{\"key\":>1}";
        assert!(JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_object()
            .is_ok());

        let str = "{\"key\"=1}";
        assert!(JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_object()
            .is_ok());

        let str = "{\"key\"=>1}";
        assert!(JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_object()
            .is_ok());

        let str = "{\"key\":=1}";
        assert!(JsonTokener::new(str).next_value().is_err());

        let str = "{\"key\":1, \"num\": 10; \"count\": 10}";
        assert!(JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_object()
            .is_ok());
    }

    #[test]
    fn tokener_empty_object() {
        let str = "{}";
        assert!(JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_object()
            .is_ok())
    }

    #[test]
    #[should_panic = "must has a key for non-empty object"]
    fn tokener_json_object_error() {
        let str = "{123}";
        JsonTokener::new(str).next_value().unwrap();
    }

    #[test]
    fn tokener_json_array() {
        let str = "[]";
        let array = JsonTokener::new(str).next_value().unwrap().as_array();
        assert!(array.is_ok());
        assert!(array.unwrap().is_empty());

        let str = "[1, 2, 3]";
        let array = JsonTokener::new(str).next_value().unwrap().as_array();
        assert!(array.is_ok());
        assert_eq!(array.unwrap().len(), 3);

        let str = "[1; 2; 3]";
        let array = JsonTokener::new(str).next_value().unwrap().as_array();
        assert!(array.is_ok());
        assert_eq!(array.unwrap().len(), 3);

        let str = "[, 1; 2; 3, ]";
        let array = JsonTokener::new(str).next_value().unwrap().as_array();
        assert!(array.is_ok());
        assert_eq!(array.unwrap().len(), 5);

        let str = "[1: 2: 3]";
        let array = JsonTokener::new(str).next_value();
        assert!(array.is_err());
    }

    #[test]
    fn tokener_string() {
        let str = "\"\n\"";
        let str = JsonTokener::new(str).next_value();
        assert!(str.is_err());

        let str = "\"\\n\\b\\f\\t\\r\\'\\\"\\\\\\/\\uD83D\\uDE01\"";
        let str = JsonTokener::new(str)
            .next_value()
            .unwrap()
            .as_string()
            .unwrap();
        assert_eq!(str, "\n\x08\x0c\t\r\'\"\\/😁");

        let str = "\"hello";
        let str = JsonTokener::new(str).next_value();
        assert!(str.is_err());
    }
}
