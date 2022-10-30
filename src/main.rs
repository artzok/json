fn main() {
    let json = std::fs::read_to_string("./test.json").unwrap();
    println!("the file json:\n{}", json);
}
