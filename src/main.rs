use json::ToJson;

fn main() {
    let json = std::fs::read_to_string("./test.json").unwrap();
    println!("{}", json::parse(&json).unwrap().pretty())
}