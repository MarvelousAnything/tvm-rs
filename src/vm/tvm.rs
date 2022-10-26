use std::fs;
use colored::Colorize;
use rand::Rng;
use serde_json::Value;
use crate::vm::builtins::BuiltIn;
use crate::vm::instruction::Instruction;

type SerdeTape = Value;

#[derive(Debug)]
pub struct Tvm {
    pub mem : [i32; 65536],
    tape: SerdeTape,
    sp : usize,
    fp : usize,
    imgnum : u32,
    butnum : u32,
    labnum : u32,
    tabnum : u32,
    edata : i32
}

pub fn read_tape(file: String) -> SerdeTape {
    let content = fs::read_to_string(file).expect("Unable to read file");
    serde_json::from_str(&content).expect("Unable to parse json")
}

impl Tvm {
    pub fn new(file: String) -> Tvm {
        Tvm {
            mem : [0; 65536],
            tape: read_tape(file),
            sp : 65535,
            fp : 65535,
            imgnum: 0,
            butnum: 0,
            labnum: 0,
            tabnum: 0,
            edata : 0
        }
    }

    pub fn print_mem(&self) {
        for (i, v) in self.mem.iter().enumerate()
            .filter(|(j, k)|
                **k != 0 || *j == self.sp || *j == self.fp) {
            if i == self.sp {
                println!("{}", format!("{}: {}", i, v).green());
            } else if i == self.fp {
                println!("{}", format!("{}: {}", i, v).red());
            } else if i == self.edata as usize {
                println!("{}", format!("{}: {}", i, v).yellow());
            } else {
                println!("{}: {}", i, v);
            }
        }
    }

    pub fn push(&mut self, val: i32) {
        self.mem[self.sp] = val;
        self.sp -= 1;
    }

    pub fn pop(&mut self) -> i32 {
        self.sp += 1;
        self.mem[self.sp]
    }

    pub fn a2s(&mut self, addr: usize) -> String {
        let mut s = String::new();
        let mut x = addr;
        while self.mem[x] != 0 {
            s.push(self.mem[x] as u8 as char);
            x += 1;
        }
        s
    }

    pub fn write_str(&mut self, addr: usize, s: String) {
        let mut x = addr;
        for c in s.chars() {
            self.mem[x] = c as i32;
            x += 1;
        }
        self.mem[x] = 0;
    }

    pub fn leval(&mut self, l: &Vec<Value>) -> i32 {
        let mut pc = 0;
        while pc >= 0 {
            pc = self.eval(l, pc);
        }
        pc
    }

    pub fn call(&mut self, n: i32) {
        let op = BuiltIn::get_builtin(n);
        match op {
            BuiltIn::IPrint => {
                let x = self.pop();
                print!("{}", x);
                self.push(0);
            }
            BuiltIn::SPrint => {
                let addr = self.pop() as usize;
                let s = self.a2s(addr);
                print!("{}", s);
                self.push(0);
            }
            BuiltIn::IRead => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Unable to read line");
                let x = input.trim().parse::<i32>().expect("Unable to parse input");
                self.push(x);
            }
            BuiltIn::SRead => {
                let a = self.pop() as usize;
                let p = self.pop();
                let mut x = String::new();
                if p == -1 {
                    println!("String input:");
                    std::io::stdin().read_line(&mut x).expect("Unable to read line");
                } else {
                    let s = self.a2s(a);
                    println!("{}", s);
                    std::io::stdin().read_line(&mut x).expect("Unable to read line");
                }
                self.write_str(a, x);
                self.push(0);
            }
            BuiltIn::NL => {
                println!();
                self.push(0);
            },
            BuiltIn::Random => {
                let n = self.pop();
                let x = rand::thread_rng().gen_range(0..n);
                self.push(x);
            }
            BuiltIn::Timer => {
                // This currently calls f after n milliseconds and is blocking. It should be on another thread.
                let to = self.pop();
                let f = self.pop();
                std::thread::sleep(std::time::Duration::from_millis(to as u64));
                self.call(f);
                self.push(0); // push the timeout number
            }
            BuiltIn::STOPTIMER => {
                // This should stop the timer thread.
                self.push(0);
            },
            BuiltIn::ALLOC => {
                let n = self.pop();
                self.push(self.edata);
                self.edata += n;
            },
            BuiltIn::FREE => {
                let _ = self.pop();
                self.push(0);
            },
            BuiltIn::I2S => {
                let a = self.pop() as usize;
                let x = self.pop();
                self.write_str(a, x.to_string());
                self.push(0);
            },
            BuiltIn::UNKNOWN => {
                if n < 0 {
                    panic!("Unknown builtin: {}", n);
                }
                let method = &self.tape[n as usize+2].clone();
                for _ in 0..method[3].as_i64().unwrap() {
                    self.push(0);
                }
                self.mem[self.sp] = self.fp as i32;
                self.fp = self.sp;
                self.sp -= 1;
                let body = method[4]
                    .as_array()
                    .unwrap();
                self.leval(body);
                let r = self.pop();
                self.sp = self.fp;
                self.fp = self.mem[self.sp] as usize;
                self.sp += (method[2].as_u64().unwrap() + method[3].as_u64().unwrap()) as usize;
                self.push(r);
            }
        }
    }

    pub fn eval(&mut self, l: &Vec<Value>, mut pc: i32) -> i32 {
        if pc >= l.len() as i32 {
            return -1;
        }
        let ir = l[pc as usize].as_i64().unwrap() as i32;
        pc += 1;
        let op = Instruction::get_instruction(ir);
        match op {
            Instruction::Push => {
                self.push(l[pc as usize].as_i64().unwrap() as i32);
                pc += 1;
            }
            Instruction::Fetch => {
                self.push(l[pc as usize].as_i64().unwrap() as i32);
                let a = self.pop();
                self.push(self.mem[a as usize]);
            }
            Instruction::Store => {
                let a = self.pop();
                let v = self.pop();
                self.mem[a as usize] = v;
            }
            Instruction::IF => {
                let x = self.pop();
                let r;
                if x != 0 {
                    r = self.leval(l[pc as usize].as_array().unwrap());
                    pc += 2
                } else {
                    pc += 1;
                    r = self.leval(l[pc as usize].as_array().unwrap());
                    pc += 1;
                }
                if r <= -2 {
                    return r;
                }
            }
            Instruction::Loop => {
                loop {
                    let r = self.leval(l[pc as usize].as_array().unwrap());
                    if r == -2 {
                        break;
                    } else if r == -3 {
                        return r;
                    }
                }
                pc  += 1;
            }
            Instruction::Break => {
                let x = self.pop();
                if x != 0 {
                    return -2;
                }
            }
            Instruction::Return => {
                return -3;
            }
            Instruction::Call => {
                self.call(l[pc as usize].as_i64().unwrap() as i32);
                pc += 1;
            }
            Instruction::FPPlus => {
                let mut a = self.pop();
                a += self.fp as i32;
                self.push(a);
            }
            Instruction::Add => {
                let x = self.pop();
                let y = self.pop();
                self.push(x + y);
            }
            Instruction::Sub => {
                let x = self.pop();
                let y = self.pop();
                self.push(y - x);
            }
            Instruction::Mul => {
                let x = self.pop();
                let y = self.pop();
                self.push(x * y);
            }
            Instruction::Div => {
                let x = self.pop();
                let y = self.pop();
                self.push(y / x);
            }
            Instruction::Mod => {
                let x = self.pop();
                let y = self.pop();
                self.push(y % x);
            }
            Instruction::Not => {
                let x = self.pop();
                self.push(!x);
            }
            Instruction::And => {
                let x = self.pop();
                let y = self.pop();
                self.push(x & y);
            }
            Instruction::OR => {
                let x = self.pop();
                let y = self.pop();
                self.push(x | y);
            }
            Instruction::Xor => {
                let x = self.pop();
                let y = self.pop();
                self.push(x ^ y);
            }
            Instruction::EQ => {
                let x = self.pop();
                let y = self.pop();
                self.push((x == y) as i32);
            }
            Instruction::Neq => {
                let x = self.pop();
                let y = self.pop();
                self.push((x != y) as i32);
            }
            Instruction::LT => {
                let x = self.pop();
                let y = self.pop();
                self.push((y < x) as i32);
            }
            Instruction::Leq => {
                let x = self.pop();
                let y = self.pop();
                self.push((y <= x) as i32);
            }
            Instruction::GT => {
                let x = self.pop();
                let y = self.pop();
                self.push((y > x) as i32);
            }
            Instruction::Geq => {
                let x = self.pop();
                let y = self.pop();
                self.push((y >= x) as i32);
            }
            Instruction::Pop => {
                self.pop();
            }
            Instruction::LShift => {
                let x = self.pop();
                let y = self.pop();
                self.push(y << x);
            }
            Instruction::RShift => {
                let x = self.pop();
                let y = self.pop();
                self.push(y >> x);
            }
            Instruction::Unknown => {
                panic!("Unknown instruction: {}", ir);
            }
        }
        pc
    }

    pub fn bootstrap(&mut self) {
        for element in self.tape[1].as_array().unwrap() {
            let i = element[0].as_u64().unwrap() as usize;
            let v = element[1].as_i64().unwrap() as i32;
            self.mem[i] = v;
        }

        self.edata = self.tape[0][1].as_i64().unwrap() as i32;
    }

    pub fn start(&mut self) {
        self.call(self.tape[0][0].as_u64().unwrap() as i32);
    }
}