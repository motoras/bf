pub mod min_interpretor;
use min_interpretor::bf;
use std::fs::File;
use std::io::stdin;
use std::io::BufWriter;
use std::io::Read;

fn read_as_bytes(name: &str) -> Vec<u8> {
    dbg!(name);
    let mut prog = Vec::new();
    let mut f_instr = File::open(name).unwrap();
    f_instr.read_to_end(&mut prog).unwrap();
    prog
}

fn main() {
    //run mandelbrot sample for the interpreter
    let name = "mandelbrot";
    let prog = read_as_bytes(&format!("src/bf/{}/{}.bf", name, name));
    let mut out = BufWriter::new(Vec::new());
    bf(&prog, &mut stdin().lock(), &mut out);
    println!("{}", std::str::from_utf8(&out.buffer()).unwrap());
}
