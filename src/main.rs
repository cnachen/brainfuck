use std::fs;
use std::env;
use std::process;
use std::path::Path;
use std::io::{self, Read};

struct Interpreter {
    mem: Vec<u8>,
    ptr: usize,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            mem: vec![0; 1024],
            ptr: 0,
        }
    }

    fn eval(&mut self, code: impl AsRef<str>) {
        let mut pc = 0;
        let program: Vec<char> = code.as_ref().chars().collect();

        let find_match = |from: char, to: char, mut pc: usize, reverse: bool| -> usize {
            let mut nested = 0;
            let mov = |pc: usize| -> usize {
                if reverse { pc - 1 } else { pc + 1 }
            };
            pc = mov(pc);
            while program[pc] != to || nested != 0 {
                if program[pc] == from { nested += 1 }
                if program[pc] == to { nested -= 1 }
                pc = mov(pc);
            }
            pc
        };

        while pc < program.len() {
            match program[pc] {
                '>' => self.ptr += 1,
                '<' => self.ptr -= 1,
                '+' => self.mem[self.ptr] += 1,
                '-' => self.mem[self.ptr] -= 1,
                '.' => {
                    print!("{}", self.mem[self.ptr] as char);
                }
                ',' => {
                    let mut buffer = [0; 1];
                    if io::stdin().read_exact(&mut buffer).is_err() {
                        process::exit(0);
                    }
                    self.mem[self.ptr] = buffer[0];
                }
                '[' => {
                    if self.mem[self.ptr] == 0 {
                        pc = find_match('[', ']', pc, false);
                    }
                }
                ']' => {
                    if self.mem[self.ptr] != 0 {
                        pc = find_match(']', '[', pc, true);
                    }
                }
                _ => {}
            }
            pc += 1;
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    if args.len() != 2 || !Path::new(&args[1]).exists() {
        eprintln!("usage: {} [FILE]", args[0]);
        process::exit(-1);
    }
    let code = fs::read_to_string(&args[1]).unwrap();
    let mut bf = Interpreter::new();
    bf.eval(code);
}
