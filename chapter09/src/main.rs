use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("chapter9.txt");

    println!("{:?} \n", f);

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("chapter9.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_errors => panic!("Problem opening the file: {:?}", other_errors),
        },
    };

    let _ = File::open("chapter9.txt").unwrap();

    let _g = File::open("chapter9_2.txt").expect("Problem opening 'chapter9_2.txt'");

    let h = File::open("chapter9_3.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}
