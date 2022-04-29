mod emulator;

fn main() {
    let path = std::env::args().nth(1).expect("no to path URCL file given");
    // read the file from the path
    let file = std::fs::read_to_string(path).expect("could not read file");
    
    emulator::emulator(file);
}


