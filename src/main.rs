use json::{JsonObject, ToJson};

fn main() {
    let json = std::fs::read_to_string("./test.json").unwrap();
    let json = JsonObject::create(&json).unwrap();
     println!("the file json:\n{}", json.pretty());
    let json = json.get("data").unwrap().opt_object_ref().unwrap().pretty();
    println!("{}", json);
}
