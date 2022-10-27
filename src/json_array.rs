use crate::{ElementType, JSONElement};

///
/// JSON [`ElementType::Array`] 类型
/// 
/// 其内部以列表形式存储 [`ElementType::Object`]、[`ElementType::Field`]
/// 
/// **Note:**
/// 
///[`JSONArray`] 内部不能直接存储 [`JSONArray`]
pub struct JSONArray {
    list: Vec<Box<dyn JSONElement>>,
}
impl JSONElement for JSONArray {
    fn get_type(&self) -> ElementType {
        ElementType::Array
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_type() {
        let object = JSONArray { list: vec![] };
        assert_eq!(object.get_type(), ElementType::Array);
    }
}
