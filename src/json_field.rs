use crate::{ElementType, JSONElement};

///
/// JSON 元素类型 Field 的值类型：
/// * 无符号整数 u128
/// * 有符号整数 i128
/// * true/false bool
/// * 浮点类型  f64
/// * 字符串 String
///
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Uint(u128),
    Int(i128),
    Bool(bool),
    Float(f64),
    String(String),
    None,
}

///
/// JSON [`ElementType::Field`] 类型
/// 
/// 其内部存储元数据，如 `String`、`Int`、`Bool` 等 
/// 
pub struct JSONField {
    value: FieldValue,
}

impl JSONElement for JSONField {
    fn get_type(&self) -> ElementType {
        ElementType::Field
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_type() {
        let object = JSONField { value: None };
        assert_eq!(object.get_type(), ElementType::Field);
    }
}
