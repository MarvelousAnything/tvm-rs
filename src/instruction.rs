use crate::callable::Caller;
use crate::frame::{Frame, FrameData};
use crate::stack::StackHolder;
use crate::state::StateResult::Exit;
use crate::state::{states, StateResult, Stateful};
use crate::tvm::Tvm;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Push {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Fetch {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Store {
        op: u32,
        name: String,
        num_operands: u32,
    },
    IF {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Loop {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Break {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Return {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Call {
        op: u32,
        name: String,
        num_operands: u32,
    },
    FPPlus {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Add {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Sub {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Mul {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Div {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Mod {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Not {
        op: u32,
        name: String,
        num_operands: u32,
    },
    And {
        op: u32,
        name: String,
        num_operands: u32,
    },
    OR {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Xor {
        op: u32,
        name: String,
        num_operands: u32,
    },
    EQ {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Neq {
        op: u32,
        name: String,
        num_operands: u32,
    },
    LT {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Leq {
        op: u32,
        name: String,
        num_operands: u32,
    },
    GT {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Geq {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Pop {
        op: u32,
        name: String,
        num_operands: u32,
    },
    LShift {
        op: u32,
        name: String,
        num_operands: u32,
    },
    RShift {
        op: u32,
        name: String,
        num_operands: u32,
    },
    Unknown(u32),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Instruction {
    pub fn get_instruction(op_code: u32) -> Instruction {
        match op_code {
            1 => Instruction::Push {
                op: 1,
                name: "push".to_string(),
                num_operands: 0,
            },
            2 => Instruction::Fetch {
                op: 2,
                name: "fetch".to_string(),
                num_operands: 0,
            },
            3 => Instruction::Store {
                op: 3,
                name: "store".to_string(),
                num_operands: 0,
            },
            4 => Instruction::IF {
                op: 4,
                name: "if".to_string(),
                num_operands: 0,
            },
            5 => Instruction::Loop {
                op: 5,
                name: "loop".to_string(),
                num_operands: 0,
            },
            6 => Instruction::Break {
                op: 6,
                name: "break".to_string(),
                num_operands: 0,
            },
            7 => Instruction::Return {
                op: 7,
                name: "return".to_string(),
                num_operands: 0,
            },
            8 => Instruction::Call {
                op: 8,
                name: "call".to_string(),
                num_operands: 0,
            },
            9 => Instruction::FPPlus {
                op: 9,
                name: "fp+".to_string(),
                num_operands: 0,
            },
            10 => Instruction::Add {
                op: 10,
                name: "+".to_string(),
                num_operands: 0,
            },
            11 => Instruction::Sub {
                op: 11,
                name: "-".to_string(),
                num_operands: 0,
            },
            12 => Instruction::Mul {
                op: 12,
                name: "*".to_string(),
                num_operands: 0,
            },
            13 => Instruction::Div {
                op: 13,
                name: "/".to_string(),
                num_operands: 0,
            },
            14 => Instruction::Mod {
                op: 14,
                name: "%".to_string(),
                num_operands: 0,
            },
            15 => Instruction::Not {
                op: 15,
                name: "!".to_string(),
                num_operands: 0,
            },
            16 => Instruction::And {
                op: 16,
                name: "&".to_string(),
                num_operands: 0,
            },
            17 => Instruction::OR {
                op: 17,
                name: "|".to_string(),
                num_operands: 0,
            },
            18 => Instruction::Xor {
                op: 18,
                name: "^".to_string(),
                num_operands: 0,
            },
            19 => Instruction::EQ {
                op: 19,
                name: "==".to_string(),
                num_operands: 0,
            },
            20 => Instruction::Neq {
                op: 20,
                name: "!=".to_string(),
                num_operands: 0,
            },
            21 => Instruction::LT {
                op: 21,
                name: "<".to_string(),
                num_operands: 0,
            },
            22 => Instruction::Leq {
                op: 22,
                name: "<=".to_string(),
                num_operands: 0,
            },
            23 => Instruction::GT {
                op: 23,
                name: ">".to_string(),
                num_operands: 0,
            },
            24 => Instruction::Geq {
                op: 24,
                name: ">=".to_string(),
                num_operands: 0,
            },
            25 => Instruction::Pop {
                op: 25,
                name: "pop".to_string(),
                num_operands: 0,
            },
            26 => Instruction::LShift {
                op: 26,
                name: "<<".to_string(),
                num_operands: 0,
            },
            27 => Instruction::RShift {
                op: 27,
                name: ">>".to_string(),
                num_operands: 0,
            },
            _ => Instruction::Unknown(op_code),
        }
    }

    pub fn op(&self) -> u32 {
        match self {
            Instruction::Push { op, .. } => *op,
            Instruction::Fetch { op, .. } => *op,
            Instruction::Store { op, .. } => *op,
            Instruction::IF { op, .. } => *op,
            Instruction::Loop { op, .. } => *op,
            Instruction::Break { op, .. } => *op,
            Instruction::Return { op, .. } => *op,
            Instruction::Call { op, .. } => *op,
            Instruction::FPPlus { op, .. } => *op,
            Instruction::Add { op, .. } => *op,
            Instruction::Sub { op, .. } => *op,
            Instruction::Mul { op, .. } => *op,
            Instruction::Div { op, .. } => *op,
            Instruction::Mod { op, .. } => *op,
            Instruction::Not { op, .. } => *op,
            Instruction::And { op, .. } => *op,
            Instruction::OR { op, .. } => *op,
            Instruction::Xor { op, .. } => *op,
            Instruction::EQ { op, .. } => *op,
            Instruction::Neq { op, .. } => *op,
            Instruction::LT { op, .. } => *op,
            Instruction::Leq { op, .. } => *op,
            Instruction::GT { op, .. } => *op,
            Instruction::Geq { op, .. } => *op,
            Instruction::Pop { op, .. } => *op,
            Instruction::LShift { op, .. } => *op,
            Instruction::RShift { op, .. } => *op,
            Instruction::Unknown(op) => *op,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Instruction::Push { name, .. } => name.clone(),
            Instruction::Fetch { name, .. } => name.clone(),
            Instruction::Store { name, .. } => name.clone(),
            Instruction::IF { name, .. } => name.clone(),
            Instruction::Loop { name, .. } => name.clone(),
            Instruction::Break { name, .. } => name.clone(),
            Instruction::Return { name, .. } => name.clone(),
            Instruction::Call { name, .. } => name.clone(),
            Instruction::FPPlus { name, .. } => name.clone(),
            Instruction::Add { name, .. } => name.clone(),
            Instruction::Sub { name, .. } => name.clone(),
            Instruction::Mul { name, .. } => name.clone(),
            Instruction::Div { name, .. } => name.clone(),
            Instruction::Mod { name, .. } => name.clone(),
            Instruction::Not { name, .. } => name.clone(),
            Instruction::And { name, .. } => name.clone(),
            Instruction::OR { name, .. } => name.clone(),
            Instruction::Xor { name, .. } => name.clone(),
            Instruction::EQ { name, .. } => name.clone(),
            Instruction::Neq { name, .. } => name.clone(),
            Instruction::LT { name, .. } => name.clone(),
            Instruction::Leq { name, .. } => name.clone(),
            Instruction::GT { name, .. } => name.clone(),
            Instruction::Geq { name, .. } => name.clone(),
            Instruction::Pop { name, .. } => name.clone(),
            Instruction::LShift { name, .. } => name.clone(),
            Instruction::RShift { name, .. } => name.clone(),
            Instruction::Unknown(_) => "unknown".to_string(),
        }
    }
}

pub trait Evaluator: Debug + Clone {
    fn do_eval(&mut self, frame: &mut Frame, in_loop: bool);
    fn get_next_frame(frame_data: &mut FrameData) -> Option<Frame>;
}

impl Evaluator for Tvm {
    fn do_eval(&mut self, frame: &mut Frame, in_loop: bool) {
        if frame.pc >= frame.data.len() {
            self.last_result = Some(Exit);
            return;
        }
        let data = &frame.data.get(frame.pc).unwrap();
        frame.pc += 1;
        match data {
            FrameData::Frame(frame) => self.frame_eval(frame.clone(), in_loop),
            FrameData::Instruction(instruction, ..) => {
                println!("Evaluating instruction: {:?}", instruction);
                match instruction {
                    Instruction::Push { .. } => {
                        let x = &frame.data[frame.pc].get_id();
                        self.push(*x);
                        frame.pc += 1;
                    }
                    Instruction::Fetch { .. } => {
                        let index = self.pop();
                        self.push(self.memory[index as usize]);
                    }
                    Instruction::Store { .. } => {
                        let value = self.pop();
                        let index = self.pop();
                        self.memory[index as usize] = value;
                    }
                    Instruction::IF { .. } => {
                        let condition = self.pop();
                        let next_frame = Tvm::get_next_frame(&mut frame.data[frame.pc])
                            .expect("could not get next frame");
                        if condition != 0 {
                            self.frame_eval(next_frame, in_loop);
                            frame.pc += 2;
                        } else {
                            frame.pc += 1;
                            self.frame_eval(next_frame, in_loop);
                            frame.pc += 1;
                        }
                        match self.last_result {
                            Some(StateResult::Break) => (),
                            Some(StateResult::Return(_)) => (),
                            _ => panic!("IF instruction did not terminate."),
                        }
                    }
                    // Evaluate the next frame until break or return is called.
                    Instruction::Loop { .. } => {
                        let mut next_frame = Tvm::get_next_frame(&mut frame.data[frame.pc])
                            .expect("could not get next frame for loop");
                        let mut loop_frame = frame.clone();
                        next_frame.name = "loop-".to_string();
                        next_frame.name.push_str(&loop_frame.name);
                        loop_frame.pc -= 1;
                        self.set_state(
                            states::Loop {
                                frame: next_frame,
                                loop_frame: frame.clone(),
                            }
                            .into(),
                        );
                    }
                    Instruction::Break { .. } => {
                        let x = self.pop();
                        if x != 0 {
                            self.last_result = Some(StateResult::Break);
                        }
                    }
                    Instruction::Return { .. } => {
                        self.last_result = Some(StateResult::Return(0));
                    }
                    Instruction::Call { .. } => {
                        let id = &frame.data[frame.pc].get_id();
                        let callable = self.get_callable(*id);
                        self.call(callable);
                        frame.pc += 1;
                    }
                    Instruction::FPPlus { .. } => {
                        let x = self.pop();
                        self.push(x + self.frame_pointer as i32);
                    }
                    Instruction::Add { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x + y);
                    }
                    Instruction::Sub { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x - y);
                    }
                    Instruction::Mul { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x * y);
                    }
                    Instruction::Div { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x / y);
                    }
                    Instruction::Mod { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x % y);
                    }
                    Instruction::Not { .. } => {
                        let x = self.pop();
                        self.push(!x);
                    }
                    Instruction::And { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x & y);
                    }
                    Instruction::OR { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x | y);
                    }
                    Instruction::Xor { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x ^ y);
                    }
                    Instruction::EQ { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x == y) as i32);
                    }
                    Instruction::Neq { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x != y) as i32);
                    }
                    Instruction::LT { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x < y) as i32);
                    }
                    Instruction::Leq { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x <= y) as i32);
                    }
                    Instruction::GT { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x > y) as i32);
                    }
                    Instruction::Geq { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push((x >= y) as i32);
                    }
                    Instruction::Pop { .. } => {
                        self.pop();
                    }
                    Instruction::LShift { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x << y);
                    }
                    Instruction::RShift { .. } => {
                        let y = self.pop();
                        let x = self.pop();
                        self.push(x >> y);
                    }
                    Instruction::Unknown(op) => panic!("Unknown instruction: {}", op),
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
