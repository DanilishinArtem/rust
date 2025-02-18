use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully");
            file
        },
        Err(error) => {
            panic!("Froblems opening the file: {:?}", error)
        },
    };
}