use clap::{command, Arg, ArgAction};
use json::{BuildConfig, ToJson};

fn main() {
    // 1. -f --file 指定格式化文件，否则从标准输入中读入
    // 2. -i --indent 指定前置格式符合
    // 3. --check_nest 检查字符串中嵌套的json并解析
    // 4. --color 显示颜色
    let matches = command!()
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Append)
                .help("read json string from file"),
        )
        .arg(
            Arg::new("indent")
                .short('i')
                .long("indent")
                .help("output format json indent string"),
        )
        .arg(
            Arg::new("check_nest")
                .long("check_nest")
                .action(ArgAction::SetTrue)
                .help("checkout nest json in string and format it"),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .action(ArgAction::SetTrue)
                .help("output with color"),
        )
        .get_matches();

    let default_indent = String::from(" ");
    let files = matches.get_many::<String>("file");
    let indent = matches
        .get_one::<String>("indent")
        .unwrap_or(&default_indent);
    let check_nest = matches.get_flag("check_nest");
    let color = matches.get_flag("color");

    if files.is_none() {
        let mut stdin = String::new();
        std::io::stdin().read_line(&mut stdin).unwrap_or_else(|_| {
            println!("read json error from stdin");
            std::process::exit(-1);
        });
        format_json(&stdin, indent, check_nest, color);
    } else {
        files.unwrap().for_each(|f| {
            let json = std::fs::read_to_string(f).unwrap_or_else(|e| {
                println!("read json error from {}: {}", f, e.to_string());
                std::process::exit(-1);
            });
            println!("{}:", f);
            format_json(&json, indent, check_nest, color);
        });
    }
}

fn format_json(json: &str, indent: &str, check_nest: bool, colored: bool) {
    let json = json::parse(&json).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(-1);
    });
    println!(
        "{}",
        json.to_json(&BuildConfig::new(true, indent, check_nest, colored))
    )
}
