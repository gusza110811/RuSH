use std::{
    self,
    env::{self},
    io::{self, Write},
    process::{Command},
};
use whoami;

fn input(stdin: &io::Stdin) -> String {
    let mut buf = String::new();
    _ = stdin.read_line(&mut buf);
    buf.pop(); // remove the newline
    return buf;
}

fn flush(stdout: &mut io::Stdout) {
    _ = stdout.flush();
}

fn parse(line: &String) -> Vec<String> {
    let split: Vec<&str> = line.split(" ").collect();
    let mut start: usize = 0;
    let mut result: Vec<String> = Vec::new();
    let mut merging = false;

    for (idx, part) in (&split).into_iter().enumerate() {
        if part.ends_with("\"") {
            merging = false;
            let mut value = split[start..idx + 1].join(" ");
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

fn exec(path: &String, args: &Vec<String>){
    let mut process = Command::new(path);
    process.args(args);
    let child = process.spawn();
    match child {
        Ok(mut c) => _ = c.wait(),
        Err(err) => println!("Failed: {}",err)
    }
}

fn shell(dir: Option<&String>) {
    let mut user: String;
    let mut parsed: Vec<String>;
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let username = whoami::username();
    let home = String::new() + "/home/" + &username;
    match dir {
        Some(dir) => _ = env::set_current_dir(dir),
        None => _ = env::set_current_dir(home),
    }
    loop {
        let cwd = env::current_dir().unwrap();
        print!("{}$ ", cwd.display());
        flush(&mut stdout);
        user = input(&stdin);
        parsed = parse(&user);
        let command = &parsed[0];

        exec(&command, &parsed[1..].to_vec());
    }
}

fn main() {
    shell(None);
}
