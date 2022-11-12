use crate::callable::Caller;
use crate::frame::{Frame, FrameData};
use crate::stack::StackHolder;
use crate::state::{StateHolder, StateResult};
use crate::tvm::Tvm;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Push = 1,
    Fetch,
    Store,
    IF,
    Loop,
    Break,
    Return,
    Call,
    FPPlus,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    And,
    OR,
    Xor,
    EQ,
    Neq,
    LT,
    Leq,
    GT,
    Geq,
    Pop,
    LShift,
    RShift = 27,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Instruction {
    pub fn get_instruction(op_code: u32) -> Instruction {
        match op_code {
            1 => Instruction::Push,
            2 => Instruction::Fetch,
            3 => Instruction::Store,
            4 => Instruction::IF,
            5 => Instruction::Loop,
            6 => Instruction::Break,
            7 => Instruction::Return,
            8 => Instruction::Call,
            9 => Instruction::FPPlus,
            10 => Instruction::Add,
            11 => Instruction::Sub,
            12 => Instruction::Mul,
            13 => Instruction::Div,
            14 => Instruction::Mod,
            15 => Instruction::Not,
            16 => Instruction::And,
            17 => Instruction::OR,
            18 => Instruction::Xor,
            19 => Instruction::EQ,
            20 => Instruction::Neq,
            21 => Instruction::LT,
            22 => Instruction::Leq,
            23 => Instruction::GT,
            24 => Instruction::Geq,
            25 => Instruction::Pop,
            26 => Instruction::LShift,
            27 => Instruction::RShift,
            _ => panic!("Invalid instruction"),
        }
    }

    pub fn op(&self) -> u32 {
        self.clone() as u32
    }

    pub fn name(&self) -> String {
        match self {
            Instruction::Push => "push".to_string(),
            Instruction::Fetch => "fetch".to_string(),
            Instruction::Store => "store".to_string(),
            Instruction::IF => "if".to_string(),
            Instruction::Loop => "loop".to_string(),
            Instruction::Break => "break".to_string(),
            Instruction::Return => "return".to_string(),
            Instruction::Call => "call".to_string(),
            Instruction::FPPlus => "fpplus".to_string(),
            Instruction::Add => "add".to_string(),
            Instruction::Sub => "sub".to_string(),
            Instruction::Mul => "mul".to_string(),
            Instruction::Div => "div".to_string(),
            Instruction::Mod => "mod".to_string(),
            Instruction::Not => "not".to_string(),
            Instruction::And => "and".to_string(),
            Instruction::OR => "or".to_string(),
            Instruction::Xor => "xor".to_string(),
            Instruction::EQ => "eq".to_string(),
            Instruction::Neq => "neq".to_string(),
            Instruction::LT => "lt".to_string(),
            Instruction::Leq => "leq".to_string(),
            Instruction::GT => "gt".to_string(),
            Instruction::Geq => "geq".to_string(),
            Instruction::Pop => "pop".to_string(),
            Instruction::LShift => "lshift".to_string(),
            Instruction::RShift => "rshift".to_string(),
        }
    }
}

pub trait Evaluator: Debug + Clone {
    fn do_eval(&mut self, frame: &mut Frame);
    fn get_next_frame(frame_data: &mut FrameData) -> Option<Frame>;
}

impl Evaluator for Tvm {
    fn do_eval(&mut self, frame: &mut Frame) {
        if frame.pc >= frame.data.len() {
            if self.state.check_in_loop() {
                // println!("loop detected");
                self.log.push_str("loop_detected\n");
                frame.pc = 0;
                self.state.set_result(StateResult::Continue)
            } else {
                // println!("program finished");
                self.log.push_str("program finished\n");
                self.state.set_result(StateResult::Exit);
            }
            return;
        }
        let data = &frame.data.get(frame.pc).unwrap();
        frame.pc += 1;
        self.state.set_result(StateResult::Continue);
        match data {
            FrameData::Frame(frame) => self.frame_eval(frame.clone()),
            FrameData::Instruction(instruction, ..) => {
                // println!("Evaluating instruction: {}", instruction);
                self.log
                    .push_str(format!("Evaluating instruction: {}\n", instruction).as_str());
                match instruction {
                    Instruction::Push => {
                        let x = &frame.data[frame.pc].get_id();
                        self.push(*x);
                        frame.pc += 1;
                    }
                    Instruction::Fetch => {
                        let index = self.pop();
                        self.push(self.memory[index as usize]);
                    }
                    Instruction::Store => {
                        let value = self.pop();
                        let index = self.pop();
                        self.memory[index as usize] = value;
                    }
                    Instruction::IF => {
                        let condition = self.pop();
                        let next_frame = Tvm::get_next_frame(&mut frame.data[frame.pc])
                            .expect("could not get next frame");
                        if condition != 0 {
                            self.frame_eval(next_frame);
                            frame.pc += 2;
                        } else {
                            frame.pc += 1;
                            self.frame_eval(next_frame);
                            frame.pc += 1;
                        }
                        match self.get_result() {
                            StateResult::Break => (),
                            StateResult::Return => (),
                            _ => panic!("IF instruction did not terminate."),
                        }
                    }
                    // Evaluate the next frame until break or return is called.
                    Instruction::Loop => {
                        let mut next_frame = Tvm::get_next_frame(&mut frame.data[frame.pc])
                            .expect("could not get next frame for loop");
                        next_frame.name = "loop-".to_string();
                        next_frame.name.push_str(&frame.name);
                        // frame.pc += 1;
                        self.frame_eval(next_frame)
                    }
                    Instruction::Break => {
                        let x = self.pop();
                        if x != 0 {
                            self.state.set_result(StateResult::Break);
                        }
                    }
                    Instruction::Return => {
                        self.state.set_result(StateResult::Return);
                    }
                    Instruction::Call => {
                        let id = &frame.data[frame.pc].get_id();
                        let callable = self.get_callable(*id);
                        self.call(callable);
                        frame.pc += 1;
                    }
                    Instruction::FPPlus => {
                        let x = self.pop();
                        self.push(x + self.frame_pointer as i32);
                    }
                    Instruction::Add => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x + y);
                    }
                    Instruction::Sub => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x - y);
                    }
                    Instruction::Mul => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x * y);
                    }
                    Instruction::Div => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x / y);
                    }
                    Instruction::Mod => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x % y);
                    }
                    Instruction::Not => {
                        let x = self.pop();
                        self.push(!x);
                    }
                    Instruction::And => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x & y);
                    }
                    Instruction::OR => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x | y);
                    }
                    Instruction::Xor => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x ^ y);
                    }
                    Instruction::EQ => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x == y) as i32);
                    }
                    Instruction::Neq => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x != y) as i32);
                    }
                    Instruction::LT => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x < y) as i32);
                    }
                    Instruction::Leq => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x <= y) as i32);
                    }
                    Instruction::GT => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x > y) as i32);
                    }
                    Instruction::Geq => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x >= y) as i32);
                    }
                    Instruction::Pop => {
                        self.pop();
                    }
                    Instruction::LShift => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x << y);
                    }
                    Instruction::RShift => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x >> y);
                    }
                }
            }
            FrameData::Callable(callable, ..) => panic!("Cannot evaluate callable: {:?}", callable),
            FrameData::Primitive(primitive) => panic!("Cannot evaluate primitive: {:?}", primitive),
        }
    }
    fn get_next_frame(frame_data: &mut FrameData) -> Option<Frame> {
        if let FrameData::Frame(frame) = frame_data {
            Some(frame.clone())
        } else {
            None
        }
    }
}
