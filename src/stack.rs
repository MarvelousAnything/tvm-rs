use crate::tvm::Tvm;

pub trait StackHolder {
    fn get_stack(&self) -> &[i32];
    fn get_stack_size(&self) -> usize;
    fn get_stack_pointer(&self) -> usize;
    fn pop(&mut self) -> i32;
    fn push(&mut self, value: i32);
    fn peek(&self) -> i32;
}

impl StackHolder for Tvm {
    fn get_stack(&self) -> &[i32] {
        &self.memory[self.stack_pointer..][1..]
    }

    fn get_stack_size(&self) -> usize {
        self.memory.len() - (self.stack_pointer + 1)
    }

    fn get_stack_pointer(&self) -> usize {
        self.stack_pointer
    }

    fn pop(&mut self) -> i32 {
        self.stack_pointer += 1;
        self.memory[self.stack_pointer]
    }

    fn push(&mut self, value: i32) {
        self.memory[self.stack_pointer] = value;
        self.stack_pointer -= 1;
    }

    fn peek(&self) -> i32 {
        self.memory[self.stack_pointer + 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_stack() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.push(2);
        tvm.push(3);
        assert_eq!(tvm.get_stack(), &[3, 2, 1]);
    }

    #[test]
    fn test_get_stack_size() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.push(2);
        tvm.push(3);
        assert_eq!(tvm.get_stack_size(), 3);
    }

    #[test]
    fn test_get_stack_pointer() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.push(2);
        tvm.push(3);
        // 65535 - 3 = 65532
        assert_eq!(tvm.get_stack_pointer(), 65532);
    }

    #[test]
    fn test_pop() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.push(2);
        tvm.push(3);
        assert_eq!(tvm.pop(), 3);
        assert_eq!(tvm.pop(), 2);
        assert_eq!(tvm.pop(), 1);
    }

    #[test]
    fn test_push() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.push(2);
        tvm.push(3);
        assert_eq!(tvm.get_stack(), &[3, 2, 1]);
    }

    #[test]
    fn test_peek() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.push(2);
        tvm.push(3);
        let sp = tvm.get_stack_pointer();
        assert_eq!(tvm.peek(), 3, "peek() should return the top of the stack");
        assert_eq!(tvm.peek(), tvm.peek(), "peek() should be equal to itself");
        assert_eq!(tvm.get_stack_pointer(), sp, "stack pointer should not change"); // stack pointer should not change
    }
}