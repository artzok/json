use json::{JsonObject, JsonValue, JsonArray};

fn main() {
    // let json = std::fs::read_to_string("./test.json").unwrap();
    // let json = JsonObject::create(&json).unwrap();
    //  println!("the file json:\n{}", json.pretty());
    // let json = json.get("data").unwrap().opt_object_ref().unwrap().pretty();
    // println!("{}", json);

    let mut json_object = JsonObject::new();
    json_object.insert("null", JsonValue::Null);
    json_object.insert("true", true);
    json_object.insert("false", false);
    json_object.insert("i8", -10_i8);
    json_object.insert("i16", -100_i16);
    json_object.insert("i32", -100_i32);
    json_object.insert("i64", -100_i64);
    json_object.insert("i128", -100_i128);
    json_object.insert("u8", 10_u8);
    json_object.insert("u16", 100_u16);
    json_object.insert("u32", 100_u32);
    json_object.insert("u64", 100_u64);
    json_object.insert("u128", 100_u128);
    json_object.insert("f32", 100.001_f32);
    json_object.insert("f64", 100.001_f64);
    json_object.insert("name", "artzok");
    let mut object = JsonObject::new();
    object.insert("key", "value");
    json_object.insert("object", object);
    let mut array = JsonArray::new();
    array.push(100);
    array.push(true);
    array.push(false);
    array.push("artzok");
    array.push(JsonObject::new());
    array.push(JsonArray::new());
    json_object.insert("array", array);
    println!("{}", json_object.to_string());
}
