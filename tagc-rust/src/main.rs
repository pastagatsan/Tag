extern crate debug;

mod tokenizer;

fn main() {
    use std::io::BufferedReader;
    use std::io::fs::File;
    let ref filename = std::os::args()[1];
    println!("COMPILING {}", filename);
    let mut file = File::open(&Path::new(filename.as_slice()));
    let string = file.read_to_string();
    println!("{:?}", tokenizer::tokenize(string.unwrap().as_slice()));
}
