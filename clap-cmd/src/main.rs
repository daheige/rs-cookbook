use clap::{Arg, ArgAction, Command};

fn main() {
    println!("Hello,clap demo");

    // 创建cmd实例
    let matches = Command::new("clap demo")
        .version("0.1.0")
        .author("daheige")
        .about("clap args parsing")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .help("a cool file"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("five less than your favorite number"),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .help("input address")
                // .required(true)
                .default_value("127.0.0.1")
                .action(ArgAction::Set),
        )
        .get_matches();

    let my_file = matches
        .get_one::<String>("file")
        .expect("file param invalid");
    println!("the file passed:{}", my_file);

    if let Some(num_str) = matches.get_one::<String>("num") {
        match num_str.parse::<i32>() {
            Ok(num) => println!("number: {}", num),
            Err(err) => println!("number invalid,err: {}", err),
        }
    }

    let address = matches
        .get_one::<String>("address")
        .expect("address invalid");
    println!("address:{}", address);
}

/*
查看帮助信息
 % ../target/debug/clap-cmd -h
Hello,clap demo
clap args parsing

Usage: clap-cmd [OPTIONS]

Options:
  -f, --file <file>        a cool file
  -n, --number <num>       five less than your favorite number
  -a, --address <address>  input address [default: 127.0.0.1]
  -h, --help               Print help information
  -V, --version            Print version information

% ../target/debug/clap-cmd -n=12 -f=input.txt
Hello,clap demo
the file passed:input.txt
number: 12

% ../target/debug/clap-cmd -n=12S -f=input.txt
Hello,clap demo
the file passed:input.txt
number invalid,err:invalid digit found in string
 */
