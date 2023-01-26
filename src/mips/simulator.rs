//!  A simulator program for the MIPS processor that demonstrates a single cycle implementation.
//! This essentially means that all implemented instructions complete in exactly one cycle.
//! 
//! Contains: alu control, alu, control unit, data memory, instruction memory, a register file, 3 multiplexors as well as 
//! a program counter
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0

use super::{
    alu::Alu, alu_control, data_memory::DataMemory, multiplexor::Multiplexor, Control,
    InstructionMemory, RegisterFile, adder::Adder, shift_left_2::{self, ShiftLeftTwo},
};

use darken_assignment1::utils::InstrRepresentable;
use ux::{u26, u5, u6};

pub struct Simulator {
    instruction_mem: Box<InstructionMemory>,
    data_mem: Box<DataMemory>,
    register_file: Box<RegisterFile>,
    alu: Box<Alu>,
    control: Box<Control>,
    pc: usize,
    write_reg_mplex: Box<Multiplexor>,
    alu_input_mplex: Box<Multiplexor>,
    data_mem_output_mplex: Box<Multiplexor>,
    beq_mplex: Box<Multiplexor>,
    jump_mplex: Box<Multiplexor>,
    shift_mplex: Box<Multiplexor>,
    jr_mplex: Box<Multiplexor>,
    adder: Box<Adder>,
}

#[derive(PartialEq, Debug)]
pub enum RunResult {
    Success,
    Completed,
    Failure(String),
}

impl Simulator {
    pub fn new(instruction_mem: Vec<Box<dyn InstrRepresentable>>) -> Simulator {
        Simulator {
            instruction_mem: InstructionMemory::load_instruction_memory(instruction_mem),
            data_mem: Box::new(DataMemory::new()),
            register_file: Box::new(RegisterFile::new()),
            alu: Box::new(Alu::new()),
            control: Box::new(Control::new()),
            pc: 0,
            write_reg_mplex: Box::new(Multiplexor::new()),
            alu_input_mplex: Box::new(Multiplexor::new()),
            data_mem_output_mplex: Box::new(Multiplexor::new()),
            beq_mplex: Box::new(Multiplexor::new()),
            jump_mplex: Box::new(Multiplexor::new()),
            jr_mplex: Box::new(Multiplexor::new()),
            shift_mplex: Box::new(Multiplexor::new()),
            adder:Box::new(Adder {  })
        }
    }

    pub fn step(&mut self) -> RunResult {
        /* FETCH */
        let mby_instr = self.instruction_mem.get_instruction_from_pc(self.pc);
        self.pc = self.adder.add(self.pc as isize as i32, 4) as usize;

        if let Some(instr) = mby_instr {

            /* DECODE */
            let mby_bits = instr.get_bits(31, 26).expect(
                format!(
                    "Failed to get high bits from instruction! Hex rep instr: {}",
                    instr.to_hex_string()
                )
                .as_str(),
            );

            let high_bit_val = u6::new(u8::from_str_radix(mby_bits.as_str(), 2).unwrap());

            let mby_funct_bits = instr.get_bits(5, 0).expect(
                format!(
                    "Failed to get lowest bits from instruction! Hex rep instr: {}",
                    instr.to_hex_string()
                )
                .as_str(),
            );
            let funct_bits_val = u6::new(u8::from_str_radix(mby_funct_bits.as_str(), 2).unwrap());

            self.control.set_output_flags(high_bit_val, funct_bits_val);

            // Set multiplex signals, except beq multiplexor
            self.write_reg_mplex.set_signal(self.control.reg_dest());
            self.alu_input_mplex.set_signal(self.control.alu_src());
            self.jump_mplex.set_signal(self.control.jump());
            self.data_mem_output_mplex
                .set_signal(self.control.mem_to_reg());
            self.shift_mplex.set_signal(self.control.shift());
            self.jr_mplex.set_signal(self.control.jump_reg());

            // have to check if control.reg_dst is set here, determines what we send to register file
            let reg_1 = self.get_reg_1(instr);
            let read_val_1 = self.register_file.read_from_register(reg_1);

            let reg_2 = self.get_reg_2(instr);
            let read_val_2 = self.register_file.read_from_register(reg_2);

            let funct = self.get_funct(instr);

            
            let alu_signal = alu_control::get_alu_signal(
                self.control.alu_op_0(),
                self.control.alu_op_1(),
                funct,
            );

            let imm = self.get_imm(instr);
 
            let shamt = self.get_shamt(instr);
            let shamt_val: u32 = shamt.into();

            let fst_alu_op = self.shift_mplex.multiplex(Some(read_val_2), read_val_1);

            let snd_alu_op = self.alu_input_mplex.multiplex(Some(imm as i32), read_val_2);

            let snd_alu_op = self
                .shift_mplex
                .multiplex(Some(shamt_val as i32), snd_alu_op);


            /* EXECUTE */
            let alu_res = self.alu.perform_op(fst_alu_op, snd_alu_op, alu_signal);

            /* MEMREAD/MEMWRITE */
            self.data_mem
                .write_word_to_address(alu_res as usize, read_val_2, self.control.mem_write())
                .expect("Failed to write word to data memory!");

            let read_data = self
                .data_mem
                .get_word(alu_res as usize, self.control.mem_read());

            let write_to_reg_val = self.data_mem_output_mplex.multiplex(read_data, alu_res);

            /* WRITE BACK */
            let reg_3 = self.get_reg_3(instr);

            let dst_reg = self
                .write_reg_mplex
                .multiplex(Some(u32::from(reg_3) as i32), u32::from(reg_2) as i32);

            self.register_file.write_to_register(
                u5::new(dst_reg as u8),
                write_to_reg_val,
                self.control.reg_write(),
            );

            let shifter = ShiftLeftTwo::new();
            let jump_target = shifter.shift(self.get_jump_target(instr).into()) as i32;

            self.beq_mplex.set_signal(self.control.branch() && self.alu.zero());
            let beq_target = self.adder.add(self.pc as isize as i32, imm as isize as i32) as i32;
            let pc_or_beq_target = self.beq_mplex.multiplex(Some(beq_target), self.pc as isize as i32);
            

            // either pc just stays the way it is or it is the new value from jumping..
            let pc_or_beq_or_jmp = self.jump_mplex.multiplex(Some(jump_target), pc_or_beq_target as i32) as usize; 
            self.pc = self.jr_mplex.multiplex(Some(read_val_1), pc_or_beq_or_jmp as isize as i32) as usize;


            if self.control.exit() {
                return RunResult::Completed;
            }
            
            RunResult::Success
        } else if (self.pc / 4) - 1 == self.instruction_mem.get_nb_instructions() {
            return RunResult::Completed;
        } else {
            let err_msg = format!(
                "Failed to fetch instruction from instruction memory with pc-value: {}
                    Number of instructions in im: {}",
                self.pc,
                self.instruction_mem.get_nb_instructions()
            );

            return RunResult::Failure(err_msg);
        }
    }

    
    fn get_reg_1(&self, instr: &Box<dyn InstrRepresentable>) -> u5 {
        let read_reg_1_bits = instr.get_bits(25, 21).unwrap();
        u5::new(u8::from_str_radix(read_reg_1_bits.as_str(), 2).unwrap())
    }

    fn get_reg_2(&self, instr: &Box<dyn InstrRepresentable>) -> u5 {
        let read_reg_2_bits = instr.get_bits(20, 16).unwrap();
        u5::new(u8::from_str_radix(read_reg_2_bits.as_str(), 2).unwrap())
    }

    fn get_reg_3(&self, instr: &Box<dyn InstrRepresentable>) -> u5 {
        let reg_3_bits = instr.get_bits(15, 11).unwrap();
        u5::new(u8::from_str_radix(reg_3_bits.as_str(), 2).unwrap())
    }

    fn get_jump_target(&self, instr: &Box<dyn InstrRepresentable>) -> u26 {
        let bits = instr
            .get_bits(25, 0)
            .expect("failed to get the lowest 26 bits from jump target");
        u26::new(u32::from_str_radix(bits.as_str(), 2).unwrap())
    }

    fn get_imm(&self, instr: &Box<dyn InstrRepresentable>) -> i16 {
        let imm_bits = instr
            .get_bits(15, 0)
            .expect("Failed to get lowest 15 bits!");

        let sign_extend: u32 = u32::from_str_radix(&imm_bits, 2).unwrap();

        sign_extend as u16 as i16
    }

    fn get_shamt(&self, instr: &Box<dyn InstrRepresentable>) -> u5 {
        let shamt_bits = instr
            .get_bits(10, 6)
            .expect("Failed to get shamt-bits from instruction");
        u5::new(u8::from_str_radix(shamt_bits.as_str(), 2).unwrap())
    }

    fn get_funct(&self, instr: &Box<dyn InstrRepresentable>) -> u6 {
        let funct_bits = instr
            .get_bits(5, 0)
            .expect("Failed to get low bits from instruction!");
        u6::new(u8::from_str_radix(funct_bits.as_str(), 2).unwrap())
    }

    /* might delete later */
    //fn fetch(&self ) -> Option<&Box<dyn InstrRepresentable>>{
    //    self.pc = self.adder.add(self.pc as isize as i32, 4) as usize;
        //self.instruction_mem.get_instruction_from_pc(&self.pc)
    //}
    pub fn get_current_pc(&self) -> usize {
        self.pc
    }

    pub fn get_registers(&self) -> Vec<(u5, i32)> {
        self.register_file.get_registers_and_values()
    }

    pub fn get_data_mem(&self) -> Vec<(u32, u8)> {
        self.data_mem.get_contents()
    }

    pub fn get_instr_mem(&self) -> Vec<(String, &Box<dyn InstrRepresentable>)> {
        self.instruction_mem.get_contents()
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.data_mem.reset_contents();
        self.register_file.reset_registers();
    }



}

#[cfg(test)]
mod tests {
    use crate::assembler::instructions::{ITypeInstruction, JTypeInstruction, RTypeInstruction, MemoryAccessTypeInstruction,
                                            NopTypeInstruction, JRTypeInstruction, get_register_number, get_register_name};
    
    use super::*;

    #[test]
    fn test_get_bits_add() {
        // should be: 00000001001010100100000000100000
        let instr = RTypeInstruction::new("add", "$t1", "$t2", "$t0").unwrap();

        let mut bits = instr.get_bits(31, 26).unwrap();
        assert_eq!(bits, "000000");

        bits = instr.get_bits(25, 21).unwrap();
        assert_eq!(bits, "01001");

        bits = instr.get_bits(20, 16).unwrap();
        assert_eq!(bits, "01010");

        bits = instr.get_bits(15, 11).unwrap();
        assert_eq!(bits, "01000");

        bits = instr.get_bits(15, 0).unwrap();
        assert_eq!(bits, "0100000000100000");
    }

    #[test]
    fn test_get_bits_addi() {
        // should be: 00100001001010000000000000000001
        let instr = ITypeInstruction::new("addi", "$t1", "$t0", "1", 0).unwrap();

        let mut bits = instr.get_bits(31, 26).unwrap();
        assert_eq!(bits, "001000");

        bits = instr.get_bits(25, 21).unwrap();
        assert_eq!(bits, "01001");

        bits = instr.get_bits(20, 16).unwrap();
        assert_eq!(bits, "01000");

        bits = instr.get_bits(15, 11).unwrap();
        assert_eq!(bits, "00000");

        bits = instr.get_bits(15, 0).unwrap();
        assert_eq!(bits, "0000000000000001");
    }

    #[test]
    fn test_get_bits_j() {
        let instr = JTypeInstruction::new("j", "12").unwrap();

        let mut bits = instr.get_bits(31, 26).unwrap();
        assert_eq!(bits, "000010");

        bits = instr.get_bits(25, 21).unwrap();
        assert_eq!(bits, "00000");

        bits = instr.get_bits(20, 16).unwrap();
        assert_eq!(bits, "00000");

        bits = instr.get_bits(15, 11).unwrap();
        assert_eq!(bits, "00000");

        bits = instr.get_bits(15, 0).unwrap();
        assert_eq!(bits, "0000000000000011");
    }

    #[test]
    fn testy_test() {
        //let instr = RTypeInstruction::new("sra", "$zero", "$t0", "$t0");
    }

    #[test]
    fn test_set_control_flags_add() { // This should probably be moved to Control? idk
        let mut ctrl = Control::new();
        let instr = RTypeInstruction::new("add", "$t1", "$t2", "$t0").unwrap();
        let bits = instr.get_bits(31, 26).unwrap();
        let op_bits = u6::new(u8::from_str_radix(bits.as_str(), 2).unwrap());
        let funct_bits = instr.get_bits(5, 0).unwrap();
        let funct_val = u6::new(u8::from_str_radix(funct_bits.as_str(), 2).unwrap());
        assert_eq!(op_bits, u6::new(0));
        ctrl.set_output_flags(op_bits, funct_val);
        assert!(ctrl.reg_dest());
        assert!(!ctrl.alu_src());
        assert!(!ctrl.mem_to_reg());
        assert!(ctrl.reg_write());
        assert!(!ctrl.mem_read());
        assert!(!ctrl.mem_write());
        assert!(!ctrl.branch());
        assert!(ctrl.alu_op_1());
        assert!(!ctrl.alu_op_0());
    }

    #[test]
    fn test_adding_negative_and_positive_numbers() {
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        let reg_num:u8 = get_register_number("$t0").unwrap().into();
        let reg_as_usize = reg_num as usize;
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "-400",0).unwrap()));
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t2", "100",0).unwrap()));
        instructions.push(Box::new(RTypeInstruction::new("add", "$t1", "$t2", "$t0").unwrap()));
        
        let mut simulator = Simulator::new(instructions);  
        
        /* Step and after that, t0 should have the number 200 */
        simulator.step();
        simulator.step();
        simulator.step();

        
        let registers = simulator.get_registers();
        let (register,number) = registers.get(reg_as_usize).unwrap();
        assert_eq!(get_register_name(*register).unwrap(), "$t0");
        assert_eq!(*number, -300);
    }

    #[test]
    fn test_sub() {
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        let reg_num:u8 = get_register_number("$t0").unwrap().into();
        let reg_as_usize = reg_num as usize;
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "100",0).unwrap()));
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t2", "300",0).unwrap()));
        instructions.push(Box::new(RTypeInstruction::new("sub", "$t1", "$t2", "$t0").unwrap()));
        
        let mut simulator = Simulator::new(instructions);  

        simulator.step();
        simulator.step();
        simulator.step();

        let registers = simulator.get_registers();
        let (register,number) = registers.get(reg_as_usize).unwrap();
        assert_eq!(get_register_name(*register).unwrap(), "$t0");
        assert_eq!(*number, -200);
    }

    #[test]
    fn test_adding_negative_numbers() {
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        let reg_num:u8 = get_register_number("$t0").unwrap().into();
        let reg_as_usize = reg_num as usize;
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "-100",0).unwrap()));
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t2", "-100",0).unwrap()));
        instructions.push(Box::new(RTypeInstruction::new("add", "$t1", "$t2", "$t0").unwrap()));
        
        let mut simulator = Simulator::new(instructions);  
        
        /* Step 3 times and after that, t0 should have the number -200 */
        simulator.step();
        simulator.step();
        simulator.step();

        
        let registers = simulator.get_registers();
        let (register,number) = registers.get(reg_as_usize).unwrap();
        assert_eq!(get_register_name(*register).unwrap(), "$t0");
        assert_eq!(*number, -200);
    }

    #[test]
    fn test_sw_and_lw(){
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        let reg_num:u8 = get_register_number("$t1").unwrap().into();
        let reg_as_usize = reg_num as usize;
        
        /* Load some stuff to register t1 and then perform sw and lw */
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "-100",0).unwrap()));
        instructions.push(Box::new(MemoryAccessTypeInstruction::new("sw", "$t1", "8", "$zero").unwrap()));
        instructions.push(Box::new(MemoryAccessTypeInstruction::new("lw", "$t1", "8", "$zero").unwrap()));

        
        let num_instructions = instructions.len();
        let mut simulator = Simulator::new(instructions);
        assert_eq!(simulator.instruction_mem.get_contents().len(),num_instructions);
        
        /* Step and after that, t1 should have the number 200 */
        simulator.step();
        let registers = simulator.get_registers();
        let (register,number) = registers.get(reg_as_usize).unwrap();
        assert_eq!(get_register_name(*register).unwrap(), "$t1");
        assert_eq!(*number, -100);
        
        /* after this step, we should have loaded the value from t1(200) to memory address 100? I guess */ 
        simulator.step();
        // check that the datamemory address is correct
        assert_eq!(simulator.data_mem.get_word(8,true).unwrap(), -100);
        // load word to t1
        simulator.step();
        let registers = simulator.get_registers();
        let (register,number) = registers.get(reg_as_usize).unwrap();
        assert_eq!(get_register_name(*register).unwrap(), "$t1");
        assert_eq!(*number, -100);
        
        
        
        simulator.reset();
        // read again and check that it is zero.
        let registers = simulator.get_registers();
        let (register,number) = registers.get(reg_as_usize).unwrap();
        assert_eq!(get_register_name(*register).unwrap(), "$t1");
        assert_eq!(*number, 0);
        
    }

    #[test] 
    fn test_jr_backward(){
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        instructions.push(Box::new(NopTypeInstruction{}));
        instructions.push(Box::new(NopTypeInstruction{}));
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "4",0).unwrap()));
        /* jump back two.. */
        instructions.push(Box::new(JRTypeInstruction::new("jr","$t1").unwrap()));
        
        let mut simulator = Simulator::new(instructions);
        for _i in 1..5 {
            simulator.step();
        }

        assert_eq!(simulator.pc,4);
        
    }

    #[test] 
    fn test_jr_forward(){
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        /* jump forward two.. */
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "8",0).unwrap()));
        instructions.push(Box::new(JRTypeInstruction::new("jr","$t1").unwrap()));
        instructions.push(Box::new(NopTypeInstruction{}));
        instructions.push(Box::new(NopTypeInstruction{}));
        instructions.push(Box::new(NopTypeInstruction{}));
        let mut simulator = Simulator::new(instructions);
        simulator.step();
        simulator.step();
        assert_eq!(simulator.pc, 8);

    }

    #[test]
    fn test_faulty_jr(){
        let mut instructions:Vec<Box<dyn InstrRepresentable>> = Vec::new();
        instructions.push(Box::new(ITypeInstruction::new("addi", "$zero", "$t1", "32",0).unwrap()));
        instructions.push(Box::new(JRTypeInstruction::new("jr","$t1").unwrap()));
        let mut simulator = Simulator::new(instructions);
        simulator.step();
        simulator.step();

        match simulator.step() {
            RunResult::Success => assert!(false), // fail
            RunResult::Completed =>  assert!(false), // fail
            RunResult::Failure(_) => assert!(true), // succeed
        }
        
    }


    

    
}
