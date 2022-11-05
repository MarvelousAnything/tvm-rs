#[cfg(test)]
mod test {
    use crate::frame::Frame;
    use crate::instruction::{Evaluator, Instruction};
    use crate::stack::StackHolder;
    use crate::state::{StateResult, TvmState};
    use crate::tvm::Tvm;

    #[test]
    fn test_push() {
        let mut tvm = Tvm::default();
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(1), vec![])
                .primitive(10)
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[10]);
    }

    #[test]
    fn test_fetch() {
        let mut tvm = Tvm::default();
        let value = 10;
        let index: usize = 7;
        tvm.memory[index] = value;
        tvm.push(index as i32);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(2), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[value]);
    }

    #[test]
    fn test_store() {
        let mut tvm = Tvm::default();
        let value = 10;
        let index: usize = 7;

        tvm.push(index as i32);
        tvm.push(value);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(3), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.memory[index], value);
    }

    #[test]
    fn test_break() {
        let mut tvm = Tvm::default();
        tvm.push(1);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(6), vec![])
                .build(),
            0,
        );
        assert!(matches!(tvm.last_result, Some(StateResult::Break)));
    }

    #[test]
    fn test_return() {
        let mut tvm = Tvm::default();
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(7), vec![])
                .build(),
            0,
        );
        assert!(matches!(tvm.last_result, Some(StateResult::Return(_))));
    }

    #[test]
    fn test_call() {
        let mut tvm = Tvm::default();
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(8), vec![])
                .callable(-101, vec![])
                .build(),
            0,
        );
        assert!(matches!(tvm.state, TvmState::Call(_)));
    }

    #[test]
    fn test_fpplus() {
        let mut tvm = Tvm::default();
        let val = 10;
        tvm.push(val);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(9), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val + tvm.frame_pointer as i32]);
    }

    #[test]
    fn test_add() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(10), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 + val2]);
    }

    #[test]
    fn test_sub() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(11), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 - val2]);
    }

    #[test]
    fn test_mul() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(12), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 * val2]);
    }

    #[test]
    fn test_div() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(13), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 / val2]);
    }

    #[test]
    fn test_mod() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(14), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 % val2]);
    }

    #[test]
    fn test_not() {
        let mut tvm = Tvm::default();
        let val = 10;
        tvm.push(val);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(15), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[!val]);
    }

    #[test]
    fn test_and() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(16), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 & val2]);
    }

    #[test]
    fn test_or() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(17), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 | val2]);
    }

    #[test]
    fn test_xor() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(18), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 ^ val2]);
    }

    #[test]
    fn test_eq() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(19), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[(val1 == val2) as i32]);
    }

    #[test]
    fn test_neq() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(20), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[(val1 != val2) as i32]);
    }

    #[test]
    fn test_lt() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(21), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[(val1 < val2) as i32]);
    }

    #[test]
    fn test_leq() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(22), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[(val1 <= val2) as i32]);
    }

    #[test]
    fn test_gt() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(23), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[(val1 > val2) as i32]);
    }

    #[test]
    fn test_geq() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(24), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[(val1 >= val2) as i32]);
    }

    #[test]
    fn test_pop() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(25), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1]);
    }

    #[test]
    fn test_lshift() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(26), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 << val2]);
    }

    #[test]
    fn test_rshift() {
        let mut tvm = Tvm::default();
        let val1 = 10;
        let val2 = 20;
        tvm.push(val1);
        tvm.push(val2);
        tvm.do_eval(
            &mut Frame::builder()
                .instruction(Instruction::get_instruction(27), vec![])
                .build(),
            0,
        );
        assert_eq!(tvm.get_stack(), &[val1 >> val2]);
    }
}
