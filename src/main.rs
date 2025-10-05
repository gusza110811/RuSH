use std::{
    self,
    io::{self, Write},
};

fn input(stdin: &io::Stdin) -> String {
    let mut buf = String::new();
    _ = stdin.read_line(&mut buf);
    buf.pop();
    return buf;
}

fn flush(stdout: &mut io::Stdout) {
    _ = stdout.flush();
}

fn parse(line: &String) -> Vec<String> {
    let split:Vec<&str> = line.split(" ").collect();
    let mut start: usize = 0;
    let mut result: Vec<String> = Vec::new();
    let mut merging = false;

    for (idx, part) in (&split).into_iter().enumerate() {
        if part.ends_with("\"") {
            merging = false;
            let mut value = split[start..idx+1].join(" ");
            value.remove(0);
            value.pop();
            result.push(value);

        } else if part.starts_with("\"") {
            merging = true;
            start = idx;

        } else if !merging {
            result.push(part.to_string())
        }
    }
    return result;
}

fn main() {
    let mut user: String;
    let mut parsed: Vec<String>;
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!(">");
        flush(&mut stdout);
        user = input(&stdin);
        parsed = parse(&user);
        println!("{:?}", parsed);
    }
}
