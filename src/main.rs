use clap::{Arg, ArgAction, command};
use json::ToJson;

fn main() {
    // 1. -f --file 指定格式化文件，否则从标准输入中读入
    // 2. -i --indent 指定前置格式符合
    let matches = command!()
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Append)
                .help("read json string from file")
        )
        .arg(
            Arg::new("indent")
                .short('i')
                .long("indent")
                .help("output format json indent string")
        )
        .get_matches();

    let default_indent = String::from(" ");
    let files = matches.get_many::<String>("file");
    let indent = matches.get_one::<String>("indent").unwrap_or(&default_indent);

    if files.is_none() {
        let mut stdin = String::new();
        std::io::stdin().read_line(&mut stdin).unwrap_or_else(|_| {
            println!("read json error from stdin");
            std::process::exit(-1);
        });
        format_json(&stdin, indent)
    } else {
        files.unwrap().for_each(|f| {
            let json = std::fs::read_to_string(f).unwrap_or_else(|e| {
                println!("read json error from {}: {}", f, e.to_string());
                std::process::exit(-1);
            });
            println!("{}:", f);
            format_json(&json, indent);
        });
    }
}

fn format_json(json: &str, indent: &str) {
    let json = json::parse(&json).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(-1);
    });
    println!("{}", json.to_json(true, indent));
}
