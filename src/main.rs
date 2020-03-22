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
        println!("{}", "|");
    }
}

fn print_tree(path: &PathBuf, depth: i32) -> io::Result<()> {
    for entry in read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            create_line(1*depth, 3);
            let x_dis = x_dis(1*depth);
            println!("{}--{:?}", x_dis, path.file_name().unwrap());
            let _ = print_tree(&path, depth+3);
        }
        else {
            let x_dis = x_dis((1*depth) + 3);
            println!("{}|", x_dis);
            println!("{}--{:?}", x_dis, path.file_name().unwrap());
        }
    }
    Ok(())
}
