//!  The alu unit of the MIPS processor. Is used to determine the operation based on control signals,
//!  as well as performing the actual operation.
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-25
//! Version: 1.0
use ux::u4;

pub struct Alu {
    zero: bool,
}

impl Alu {
    pub fn new() -> Alu {
        return Alu { zero: false };
    }

    pub fn perform_op(&mut self, operand_1: i32, operand_2: i32, alu_signal: u4) -> i32 {
        let op = Alu::get_operation_from_signal(alu_signal);
        let res = op(operand_1, operand_2);
        self.zero = res == 0;
        res
    }

    fn get_operation_from_signal(signal: u4) -> Box<dyn Fn(i32, i32) -> i32> {
        let signal_val: u8 = signal.into();
        match signal_val {
            0 => Box::new(|x, y| x & y),
            1 => Box::new(|x, y| x | y),
            2 => Box::new(|x, y| x + y),
            6 => Box::new(|x, y| x - y),
            7 => Box::new(|x, y| if x < y { 1 } else { 0 }),
            12 => Box::new(|x, y| !(x | y)),
            13 => Box::new(|x, y| x >> y), 
            14 => Box::new(|x, y| ((x as u32) >> y) as i32),
            _ => panic!("Invalid control signal sent to ALU!"),
        }
    }

    pub fn zero(&self) -> bool {
        self.zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_op_from_signal() {
        let operand_1 = 1;
        let operand_2 = 2;

        let mut op = Alu::get_operation_from_signal(u4::new(0));
        assert_eq!(0, op(operand_1, operand_2));

        op = Alu::get_operation_from_signal(u4::new(1));
        assert_eq!(3, op(operand_1, operand_2));

        op = Alu::get_operation_from_signal(u4::new(2));
        assert_eq!(3, op(operand_1, operand_2));

        op = Alu::get_operation_from_signal(u4::new(6));
        assert_eq!(-1, op(operand_1, operand_2));

        op = Alu::get_operation_from_signal(u4::new(7));
        assert_eq!(1, op(operand_1, operand_2));

        op = Alu::get_operation_from_signal(u4::new(12));
        assert_eq!(-4, op(operand_1, operand_2)); // dealing with i32's here
    }

    #[test]
    fn test_perform_and() {
        let mut alu = Alu::new();
        let operand_1 = 2; // 0010
        let operand_2 = 4; // 0100
        let res = alu.perform_op(operand_1, operand_2, u4::new(0));
        assert_eq!(0, res);
        assert!(alu.zero);
    }

    #[test]
    fn test_perform_or() {
        let mut alu = Alu::new();
        let operand_1 = 2; // 0010
        let operand_2 = 4; // 0100
        let res = alu.perform_op(operand_1, operand_2, u4::new(1));
        assert_eq!(6, res);
        assert!(!alu.zero);
    }

    #[test]
    fn test_perform_add() {
        let mut alu = Alu::new();
        let operand_1 = 2; // 0010
        let operand_2 = 4; // 0100
        let res = alu.perform_op(operand_1, operand_2, u4::new(2));
        assert_eq!(6, res);
        assert!(!alu.zero);
    }

    #[test]
    fn test_perform_sub() {
        let mut alu = Alu::new();
        let operand_1 = 2; // 0010
        let operand_2 = 4; // 0100
        let res = alu.perform_op(operand_1, operand_2, u4::new(6));
        assert_eq!(-2, res);
        assert!(!alu.zero);
    }

    #[test]
    fn test_perform_slt() {
        let mut alu = Alu::new();
        let operand_1 = 2; // 0010
        let operand_2 = 4; // 0100
        let slt_sig = u4::new(7);
        let res = alu.perform_op(operand_1, operand_2, slt_sig);
        assert_eq!(1, res);
        assert!(!alu.zero());

        let res = alu.perform_op(operand_2, operand_1, slt_sig);
        assert_eq!(res, 0);
        assert!(alu.zero());
    }

    #[test]
    fn test_perform_nor() {
        let mut alu = Alu::new();
        let operand_1 = 2; // 0010
        let operand_2 = 4; // 0100
        let res = alu.perform_op(operand_1, operand_2, u4::new(12));
        assert_eq!(-7, res);
        assert!(!alu.zero);
    }

    #[test]
    fn test_add_neg() {
        let mut alu = Alu::new();
        let operand_1 = 8;
        let operand_2 = -4;
        let res = alu.perform_op(operand_1, operand_2, u4::new(2));
        assert_eq!(4, res);
        assert!(!alu.zero);

        let operand_1 = -8;
        let operand_2 = 4;
        let res = alu.perform_op(operand_1, operand_2, u4::new(2));
        assert_eq!(-4, res);
        assert!(!alu.zero);

        let operand_1 = -100;
        let operand_2 = 100;
        let res = alu.perform_op(operand_1, operand_2, u4::new(2));
        assert_eq!(res, 0);
        assert!(alu.zero);
    }

    #[test]
    fn test_perform_sra() {
        // Arithmetic shift, will shift in sign bit from left
        let mut alu = Alu::new();
        let sra_sig = u4::new(13);

        let operand = 24;
        let shamt = 2;
        let res = alu.perform_op(operand, shamt, sra_sig);
        assert_eq!(res, 6);
        assert!(!alu.zero);

        let operand = -1;
        let shamt = 1;
        let res = alu.perform_op(operand, shamt, sra_sig);
        assert_eq!(res, -1);
        assert!(!alu.zero);

        let operand = -128;
        let shamt = 4;
        let res = alu.perform_op(operand, shamt, sra_sig);
        assert_eq!(res, -8);
        assert!(!alu.zero);
    }

    #[test]
    fn test_perform_srl() {
        // Logical shift, shift in zeroes
        let mut alu = Alu::new();
        let sla_sig = u4::new(14);

        let operand = 24;
        let shamt = 2;
        let res = alu.perform_op(operand, shamt, sla_sig);
        assert_eq!(res, 6);
        assert!(!alu.zero);

        let operand = -1;
        let shamt = 1;
        let res = alu.perform_op(operand, shamt, sla_sig);
        assert_eq!(res, 2147483647);
        assert!(!alu.zero);

        let operand = -128;
        let shamt = 4;
        let res = alu.perform_op(operand, shamt, sla_sig);
        assert_eq!(res, 268435448);
        assert!(!alu.zero);
    }

    #[test]
    fn test_sub() {
        let mut alu = Alu::new();
        let sub_sig = u4::new(6);

        let op1 = 50;
        let op2 = 50;
        let res = alu.perform_op(op1, op2, sub_sig);
        assert_eq!(res, 0);
        assert!(alu.zero());
    }
}
