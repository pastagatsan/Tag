mod tokenizer;

fn main() {
    use std::io::BufferedReader;
    use std::io::fs::File;
    let ref filename = std::os::args()[1];
    println!("COMPILING {}", filename);
    let file = File::open(&Path::new(filename.as_slice()));
    let mut reader = BufferedReader::new(file);
    for (lnum, line) in reader.lines().enumerate() {
        print!("{} {}", lnum, line.unwrap());
    }
}
