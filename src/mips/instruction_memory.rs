//!  Instructionmemory for the mips simulator. Built as a dynamic vector of InstrRepresentable, which can be found in 
//! utils, which is a trait that any instruction type struct can implement.
//!  
//! File: instructionMemory.rs
//! Author: mai21asm, c19hln
//! Since: 2022-12-05
//! Version: 1.0
//!

use darken_assignment1::utils::InstrRepresentable;

//#[derive(Debug)]
pub struct InstructionMemory {
    instructions: Vec<Box<dyn InstrRepresentable>>,
}

impl InstructionMemory {
    #[allow(unused)]
    pub fn new() -> InstructionMemory {
        InstructionMemory {
            instructions: Vec::new(),
        }
    }

    pub fn load_instruction_memory(
        instrs: Vec<Box<dyn InstrRepresentable>>,
    ) -> Box<InstructionMemory> {
        Box::new(InstructionMemory {
            instructions: instrs,
        })
    }

    pub fn get_instruction_from_pc(&self, pc: usize) -> Option<&Box<dyn InstrRepresentable>> {
        let maybe_instruction = self.instructions.get(pc / 4);
        if maybe_instruction.is_none() {
            return None;
        }

        Some(maybe_instruction.unwrap())
    }

    /// used in testing
    #[allow(unused)]
    pub fn get_instruction_from_index(&self, idx: usize) -> Option<&Box<dyn InstrRepresentable>> {
        self.instructions.get(idx)
    }

    pub fn get_nb_instructions(&self) -> usize {
        self.instructions.len()
    }

    pub fn get_contents(&self) -> Vec<(String, &Box<dyn InstrRepresentable>)> {
        let mut v = Vec::new();
        for (idx, instr) in self.instructions.iter().enumerate() {
            let addr = idx * 4;
            v.push((addr.to_string(), instr));
        }
        v
    }
}

#[cfg(test)]
mod tests {

    use darken_assignment1::utils::InstrRepresentable;

    use super::InstructionMemory;

    use crate::assembler::instructions::{ITypeInstruction, RTypeInstruction};

    #[test]
    pub fn test_get_instruction() {
        let mut v = Vec::new();

        let add_instr: Box<dyn InstrRepresentable> = Box::new(
            RTypeInstruction::new("add", "$t1", "$t2", "$t0")
                .expect("Could not create RTypeInstruction"),
        );
        let addi_instr = Box::new(
            ITypeInstruction::new("addi", "$zero", "$t1", "1", 0)
                .expect("Could not create ITypeInstruction"),
        );

        v.push(add_instr);
        v.push(addi_instr);

        let instruction_memory = InstructionMemory::load_instruction_memory(v);

        assert_eq!(
            instruction_memory
                .get_instruction_from_index(0)
                .unwrap()
                .to_hex_string(),
            "0x012a4020".to_string()
        );
    }

    #[test]
    pub fn test_get_instruction_from_pc() {
        let mut v = Vec::new();

        let add_instr: Box<dyn InstrRepresentable> = Box::new(
            RTypeInstruction::new("add", "$t1", "$t2", "$t0")
                .expect("Could not create RTypeInstruction"),
        );

        let addi_instr = Box::new(
            ITypeInstruction::new("addi", "$zero", "$t1", "1", 0)
                .expect("Could not create ITypeInstruction"),
        );

        v.push(add_instr);
        v.push(addi_instr);

        let instruction_memory = InstructionMemory::load_instruction_memory(v);

        assert_eq!(
            instruction_memory
                .get_instruction_from_pc(0)
                .unwrap()
                .to_hex_string(),
            "0x012a4020".to_string()
        );
        assert_eq!(
            instruction_memory
                .get_instruction_from_pc(4)
                .unwrap()
                .to_hex_string(),
            "0x20090001".to_string()
        );
        assert!(instruction_memory.get_instruction_from_pc(8).is_none());
    }
}
