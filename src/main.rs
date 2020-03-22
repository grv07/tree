use colored::*;
use std::env;
use std::fs::*;
use std::io;
use std::path::*;

fn main() -> io::Result<()> {
    let path = env::current_dir().unwrap();
    println!("{:?}", path);
    print_tree(&path, 0)?;
    Ok(())
}

fn x_dis(x: i32) -> std::string::String {
    let mut s = String::new();
    for _ in 1..x {
        s.push_str(" ");
    }
    s
}

fn create_line(x: i32, y: i32) {
    for _ in 1..y {
        print!("{}", x_dis(x));
        println!("{}", "|".blue());
    }
}

fn print_tree(path: &PathBuf, depth: i32) -> io::Result<()> {
    for entry in read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            create_line(depth, 2);
            let x_dis = x_dis(depth);
            let t = format!("{}--{:?}", x_dis, path.file_name().unwrap());
            println!("{}", t.blue());
            let _ = print_tree(&path, depth + 3);
        } else {
            let x_dis = x_dis((depth) + 3);
            let file_branch = format!("{}--{:?}", x_dis, path.file_name().unwrap());
            println!("{}{}", x_dis, "|".green());
            println!("{}", file_branch.green());
        }
    }
    Ok(())
}
