fn main() {
    // let mut args = std::env::args();
    // args.next(); // ingore program name
    // let name = args.next().unwrap_or_else(|| {
    //     eprintln!("Usage: json filename true/false");
    //     exit(1);
    // });

    // let pretty = args.next().unwrap_or_else(|| {
    //     eprintln!("Usage: json filename true/false");
    //     exit(1);
    // });
    // let pretty = pretty.parse::<bool>().unwrap_or_else(|_| {
    //     exit(1);
    // });

    // if let Ok(str) = std::fs::read_to_string(name) {
    //     let json = json::parse(&str).unwrap();
    //     let json = if pretty {
    //         json.pretty()
    //     } else {
    //         json.to_string()
    //     };
    //     println!("the file json:\n{}", json);
    // }

    // let str = "json.put("中国\uD83D\uDE39", str)"

    let str = "'\\\\, \\\", \\f, \\t, \\n, \\b, \\r \\'\"'";
    let json = json::parse(str).unwrap();
    println!("{}", json)
}
