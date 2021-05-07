use std::io::BufRead;
use std::io::Write;

pub fn bf<R: BufRead, W: Write>(inst: &[u8], inp: &mut R, out: &mut W) {
    let mut regs = vec![0; 16];
    let mut reg = 0;
    let mut pos = 0usize;
    let mut loops: Vec<(usize, usize, usize)> = Vec::new();
    while pos < inst.len() {
        //dbg!(pos, inst[pos] as char, reg, regs[reg]);
        let mut jump = 1usize;
        match inst[pos] {
            43 => regs[reg] += 1, //+
            45 => regs[reg] -= 1, //-
            60 => reg -= 1,       //<
            62 => {
                reg += 1;
                while reg >= regs.len() {
                    regs.push(0);
                }
            } //>
            44 => {
                let mut line = String::new();
                inp.read_line(&mut line).expect("failed to read input.");
                regs[reg] = line.trim().parse().expect("invalid input");
            } //,
            46 => {
                write!(out, "{}", regs[reg] as u8 as char).unwrap();
            } //.
            91 => {
                if loops.len() == 0 || loops[loops.len() - 1].0 != pos {
                    let start = pos;
                    let rg = reg;
                    let mut end = pos;
                    let mut open = 0;
                    loop {
                        if inst[end] == 91 {
                            open += 1;
                        } else if inst[end] == 93 {
                            open -= 1;
                            if open == 0 {
                                break;
                            }
                        }
                        end += 1;
                    }
                    loops.push((start, end, rg));
                }
                let crt_loop = loops[loops.len() - 1];
                if regs[reg] == 0 {
                    pos = crt_loop.1 + 1;
                    jump = 0;
                    loops.pop();
                }
            } //[
            93 => {
                if loops.len() == 0 || loops[loops.len() - 1].1 != pos {
                    panic!("Invalid BF program");
                }
                let crt_loop = loops[loops.len() - 1];
                if regs[reg] == 0 {
                    loops.pop();
                } else {
                    pos = crt_loop.0;
                    jump = 0;
                }
            } //]
            _ => (),
        }
        pos += jump;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::stdin;
    use std::io::BufReader;
    use std::io::BufWriter;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn add() {
        do_test("add");
    }

    #[test]
    fn hello_world() {
        do_test("hw");
    }

    #[test]
    fn mandelbrot() {
        do_test("mandelbrot");
    }

    #[test]
    fn euler() {
        do_test("euler");
    }

    #[test]
    fn bw() {
        do_test("bw");
    }

    fn do_test(name: &str) {
        let prog = read_as_bytes(&format!("src/bf/{}/{}.bf", name, name));
        let inp_name = format!("src/bf/{}/{}.in", name, name);
        let input = Path::new(&inp_name);
        let mut out = BufWriter::new(Vec::new());
        let mut inp;
        if input.exists() {
            inp = BufReader::new(File::open(input).unwrap());
            bf(&prog, &mut inp, &mut out);
        } else {
            bf(&prog, &mut stdin().lock(), &mut out);
        }
        let bytes = out.into_inner().unwrap();
        let exp = read_as_bytes(&format!("src/bf/{}/{}.out", name, name));
        assert_eq!(exp, bytes);
    }

    fn read_as_bytes(name: &str) -> Vec<u8> {
        dbg!(name);
        let mut prog = Vec::new();
        let mut f_instr = File::open(name).unwrap();
        f_instr.read_to_end(&mut prog).unwrap();
        prog
    }
}
