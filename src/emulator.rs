use regex;
use std::collections::HashMap;
fn main() {}

struct Instruction {
    opcode: u8,
    operands: std::vec::Vec<Opperand>,
}
struct Opperand {
    type_: OppType,
    value: u8,
}
enum OppType {
    REGISTER,
    IMMEDIATE,
    MEMORY
}

struct Register {
    value: u8,
}
struct Memory {
    value: u8,
}
enum Headers {
    BITS,
    MINREG,
    MINHEAP
}

pub fn emulator(code: String) {
    // Parse the headers
    let mut headers = parse_headers(code);
    
}
fn parse_headers(code: String) -> HashMap<&'static str, &'static str>{
    // Headers look like:
    // MINREG == 5
    // BITS == 8
    // MINHEAP 33_00
    // now parse the headers
    let header_types = [
        "MINREG",
        "BITS",
        "MINHEAP",
    ];
    let header_regex = regex::Regex::new(r"^(?P<header>\w+) == (?P<value>\w+)$").unwrap();
    let mut headers = std::collections::HashMap::new();
    for line in code.lines() {
        let caps = header_regex.captures(line).unwrap();
        let header = caps.name("header").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();
        headers.insert(header, value);
    }
    return &headers;
}