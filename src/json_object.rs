use crate::{ElementType, ErrorKind, JSONElement, Result};

use std::{borrow::Borrow, collections::HashMap, fmt::Display, hash::Hash};

///
/// JSON [`ElementType::Object`] 元素类型
///
/// 其内部以 `key-value` 形式存储 [`ElementType::Object`]、
/// [`ElementType::Array`]、[`ElementType::Field`]
///
pub struct JSONObject {
    map: HashMap<String, Box<dyn JSONElement>>,
}

impl JSONElement for JSONObject {
    fn get_type(&self) -> ElementType {
        ElementType::Object
    }
}

impl JSONObject {
    ///
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the [`String`] type, but
    /// [`Hash`] and [`Eq`] on the borrowed form **must** match those for
    /// the [`String`] type.
    ///
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Box<dyn JSONElement>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq + Display,
    {
        self.map.get(key)
    }

    ///
    ///
    pub fn get_i32<Q: ?Sized>(&self, key: &Q) -> Result<i32>
    where
        String: Borrow<Q>,
        Q: Hash + Eq + Display,
    {
        let value = self.get(key);

        // if not found return Error
        if let Some(v) = value {
            match v.get_type() {
                ElementType::Array |
                ElementType::Object => {
                    return Result::Err(ErrorKind::TypeError.into());
                }
                ElementType::Field => {
                    let f = v.as_ref();
                }
            }
        } else {
            return Result::Err(ErrorKind::NotFound.into());
        }

        Result::Ok(0)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_type() {
        let object = JSONObject {
            map: HashMap::new(),
        };
        assert_eq!(object.get_type(), ElementType::Object);
    }
}
