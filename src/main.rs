use std::env;
use std::fs::*;
use std::io;
use std::path::*;

fn main() -> io::Result<()> { 
    let path = env::current_dir().unwrap();
    println!("{:?}", path);
    print_tree(&path)?;
    Ok(())
}

fn print_tree(path: &PathBuf) -> io::Result<()> {
    // let mut dir = "";
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("{:?}", path.file_name().unwrap());
            print_tree(&path)?;
        }
        else {
            println!("   |");
            println!("   -----------{:?}", path.file_name().unwrap());
        }
    }
    Ok(())
}
