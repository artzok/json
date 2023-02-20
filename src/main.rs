use json::ToJson;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap_or_else(|_| {
        println!("读入数据出错");
        std::process::exit(-1);
    });
    let json = json::parse(&line).unwrap_or_else(|e| {
        println!("parse json failed: {}", e);
        std::process::exit(-1);
    });
    println!("{}", json.pretty());
}