use json::{JsonObject, ToJson};

fn main() {
    let json = std::fs::read_to_string("./test.json").unwrap();
    let json = JsonObject::create(&json).unwrap();
    println!("the file json:\n{}", json.pretty());

    json.get("data").unwrap().as_object_ref().unwrap().unwrap().pretty();
}
