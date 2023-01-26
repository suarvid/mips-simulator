//!  The control unit of the MIPS processor. It is responsible for setting the control signals
//! so that each instruction is set properly. 
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-25
//! Version: 1.0
use ux::{u6, u13};

pub struct Control {
    exit: bool,
    shift: bool,
    jump_reg: bool,
    jump: bool,
    reg_dest: bool,
    branch: bool,
    mem_read: bool,
    mem_to_reg: bool,
    alu_op_0: bool,
    alu_op_1: bool,
    mem_write: bool,
    alu_src: bool,
    reg_write: bool,
}

impl Control {
    pub fn new() -> Control {
        Control {
            exit: false,
            shift: false,
            jump_reg: false,
            jump: false,
            reg_dest: false,
            branch: false,
            mem_read: false,
            mem_to_reg: false,
            alu_op_0: false,
            alu_op_1: false,
            mem_write: false,
            alu_src: false,
            reg_write: false,
        }
    }

    /// Mutates the state of the Control to
    /// set output bits according to the given op_bits
    pub fn set_output_flags(&mut self, op_bits: u6, funct_bits: u6) {
        let op_bits_val: u8 = op_bits.into();
        match op_bits_val {
            0 => {
                let funct_bits_val: u8 = funct_bits.into();
                match funct_bits_val {
                    8 => self.set_output_flags_to_pattern(u13::new(1024)), // jr
                    3 => self.set_output_flags_to_pattern(u13::new(2338)), // sra 
                    2 => self.set_output_flags_to_pattern(u13::new(2338)),// srl
                    _ => self.set_output_flags_to_pattern(u13::new(290))
                }
            },
            35 => self.set_output_flags_to_pattern(u13::new(240)),
            43 => self.set_output_flags_to_pattern(u13::new(136)),
            4 => self.set_output_flags_to_pattern(u13::new(5)), 
            8 => self.set_output_flags_to_pattern(u13::new(160)), // this was 161, but that set alu_op_0 for addi, which we probably don't want
            13 => self.set_output_flags_to_pattern(u13::new(163)), 
            2 => self.set_output_flags_to_pattern(u13::new(512)), 
            63 => self.set_output_flags_to_pattern(u13::new(4096)), // exit
            _ => panic!(
                "Got invalid input pattern to Control! Got value {}",
                op_bits_val
            ),
        }
    }
    // ori: 0010100011
    fn set_output_flags_to_pattern(&mut self, output_pattern: u13) {
        let output_pattern_val: u16 = output_pattern.into();
        self.exit = output_pattern_val & 4096 > 0;
        self.shift = output_pattern_val & 2048 > 0;
        self.jump_reg = output_pattern_val & 1024 > 0;
        self.jump = output_pattern_val & 512 > 0;
        self.reg_dest = output_pattern_val & 256 > 0;
        self.alu_src = output_pattern_val & 128 > 0;
        self.mem_to_reg = output_pattern_val & 64 > 0;
        self.reg_write = output_pattern_val & 32 > 0;
        self.mem_read = output_pattern_val & 16 > 0;
        self.mem_write = output_pattern_val & 8 > 0;
        self.branch = output_pattern_val & 4 > 0;
        self.alu_op_1 = output_pattern_val & 2 > 0;
        self.alu_op_0 = output_pattern_val & 1 > 0;
    }

    pub fn reg_dest(&self) -> bool {
        self.reg_dest
    }

    pub fn branch(&self) -> bool {
        self.branch
    }

    pub fn mem_read(&self) -> bool {
        self.mem_read
    }

    pub fn mem_to_reg(&self) -> bool {
        self.mem_to_reg
    }

    pub fn alu_op_0(&self) -> bool {
        self.alu_op_0
    }

    pub fn alu_op_1(&self) -> bool {
        self.alu_op_1
    }

    pub fn mem_write(&self) -> bool {
        self.mem_write
    }

    pub fn alu_src(&self) -> bool {
        self.alu_src
    }

    pub fn reg_write(&self) -> bool {
        self.reg_write
    }

    pub fn jump(&self) -> bool {
        self.jump
    }

    pub fn jump_reg(&self) -> bool {
        self.jump_reg
    }

    pub fn shift(&self) -> bool {
        self.shift
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_flags_memory_type_sw() {
        let mut control = Control::new();
        let sw_op = u6::new(43);
        control.set_output_flags(sw_op, u6::new(0));
        // reg dest is dont care
        assert!(control.alu_src);
        // mem to reg is dont care
        assert!(!control.reg_write);
        assert!(!control.mem_read);
        assert!(control.mem_write);
        assert!(!control.branch);
        assert!(!control.alu_op_0);
        assert!(!control.alu_op_1);
    }
    #[test]
    fn test_output_flags_memory_type_lw() {
        let mut control = Control::new();
        let lw_op_bits = u6::new(35);
        control.set_output_flags(lw_op_bits, u6::new(0));

        assert!(control.alu_src);
        assert!(control.mem_to_reg);
        assert!(control.reg_write);
        assert!(control.mem_read);

        assert!(!control.reg_dest);
        assert!(!control.mem_write);
        assert!(!control.branch);
        assert!(!control.alu_op_0);
        assert!(!control.alu_op_1);
    }

    #[test]
    fn test_output_flags_r_type() {
        let mut control = Control::new();
        let op_bits = u6::new(0);
        control.set_output_flags(op_bits, u6::new(0));
        // assert true signals
        assert!(control.reg_dest());
        assert!(control.reg_write());
        assert!(control.alu_op_1());

        // rest should be false
        assert!(!control.alu_op_0());
        assert!(!control.branch());
        assert!(!control.jump());
        assert!(!control.mem_read());
        assert!(!control.mem_to_reg());
        assert!(!control.mem_write());
    }

    #[test]
    fn test_output_flags_addi() {
        let mut control = Control::new();
        let addi_op_bits = u6::new(8);
        control.set_output_flags(addi_op_bits, u6::new(0));

        assert!(control.reg_write());
        assert!(control.alu_src());

        assert!(!control.reg_dest());
        assert!(!control.jump());
        assert!(!control.branch());
        assert!(!control.mem_read());
        assert!(!control.mem_write());
        assert!(!control.alu_op_0());
        assert!(!control.alu_op_1());
        assert!(!control.mem_to_reg());
    }

    #[test]
    fn test_output_flags_ori() {
        let mut control = Control::new();
        let ori_op_bits = u6::new(13);
        control.set_output_flags(ori_op_bits, u6::new(0));

        assert!(control.reg_write());
        assert!(control.alu_src());
        assert!(control.alu_op_0());
        assert!(control.alu_op_1());

        assert!(!control.reg_dest());
        assert!(!control.jump());
        assert!(!control.branch());
        assert!(!control.mem_read());
        assert!(!control.mem_write());
        assert!(!control.mem_to_reg());
    }

    #[test]
    fn test_output_flags_sra() {
        let mut control = Control::new();
        let sra_op_bits = u6::new(0);
        let sra_funct_bits = u6::new(3);
        control.set_output_flags(sra_op_bits, sra_funct_bits);

        assert!(control.alu_op_1()); 
        assert!(control.reg_dest()); 
        assert!(control.reg_write());
        assert!(control.shift());
        
        assert!(!control.alu_op_0());
        assert!(!control.branch());
        assert!(!control.jump());
        assert!(!control.jump_reg());
        assert!(!control.mem_read());
        assert!(!control.mem_to_reg());
        assert!(!control.mem_write());
    }


    #[test]
    fn test_output_flags_srl() {
        let mut control = Control::new();
        let srl_op_bits = u6::new(0);
        let srl_funct_bits = u6::new(2);
        control.set_output_flags(srl_op_bits, srl_funct_bits);

        assert!(control.alu_op_1());
        assert!(control.reg_dest());
        assert!(control.reg_write());
        assert!(control.shift());
        
        assert!(!control.alu_op_0());
        assert!(!control.branch());
        assert!(!control.jump());
        assert!(!control.jump_reg());
        assert!(!control.mem_read());
        assert!(!control.mem_to_reg());
        assert!(!control.mem_write());
    }

    #[test]
    fn test_output_flags_slt() {
        let mut control = Control::new();
        let slt_op_bits = u6::new(0);
        let slt_func_bits = u6::new(42);
        control.set_output_flags(slt_op_bits, slt_func_bits);

        assert!(control.reg_dest());
        assert!(control.alu_op_1());
        assert!(control.reg_write());

        assert!(!control.alu_op_0());
        assert!(!control.jump());
        assert!(!control.branch());
        assert!(!control.mem_read());
        assert!(!control.mem_write());
        assert!(!control.mem_to_reg());
        assert!(!control.alu_src());
        assert!(!control.exit());
        assert!(!control.shift());
        assert!(!control.jump_reg());
    }

}
