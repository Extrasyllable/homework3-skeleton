use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello 341! TEST hiii");
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("foo.txt reads: {contents}");
    Ok(())
}
