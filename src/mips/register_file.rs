//!  The register file unit storing values in registers 0-31 in the MIPS processor. 
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0
use std::collections::HashMap;
use ux::u5;

pub struct RegisterFile {
    registers: HashMap<u5, i32>,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {

        let mut registers: HashMap<u5,i32> = HashMap::new();
        for i in 0..31 as u8 {
            let index = u5::new(i);
            registers.insert(index, 0);
        }
        RegisterFile {
            registers,
        }
    }

    pub fn write_to_register(&mut self, write_reg: u5, write_data: i32, reg_write_sig: bool) {
        if reg_write_sig && write_reg != u5::new(0) {
            if write_reg < u5::new(0) || write_reg > u5::new(31) {
                panic!("incorrect write reg!");
            }
            self.registers.insert(write_reg, write_data);
        }
    }

    pub fn read_from_register(&mut self, read_reg: u5) -> i32 {
        if read_reg < u5::new(0) || read_reg > u5::new(31) {
            panic!("Got invalid register to read from! read_reg: {}", read_reg)
        }
        *self.registers.get(&read_reg).unwrap_or(&0)
    }

    /// might need it later
    #[allow(unused)]
    pub fn read_from_registers(&mut self, read_reg_1: u5, read_reg_2: u5) -> (i32, i32) {
        if read_reg_1 < u5::new(0) || read_reg_1 > u5::new(31) {
            panic!(
                "Got invalid register to read from! read_reg: {}",
                read_reg_1
            );
        }

        if read_reg_2 < u5::new(0) || read_reg_2 > u5::new(31) {
            panic!(
                "Got invalid register to read from! read_reg: {}",
                read_reg_2
            );
        }

        let val_1 = *self.registers.get(&read_reg_1).expect(&format!(
            "Got invalid register to read from! read_reg_1: {}",
            read_reg_1
        ));
        let val_2 = *self.registers.get(&read_reg_2).expect(&format!(
            "Got invalid register to read from! read_reg_2: {}",
            read_reg_2
        ));
        (val_1, val_2)
    }

    pub fn get_registers_and_values(&self) -> Vec<(u5, i32)>{
        let mut register_vec:Vec<(u5, i32)> = Vec::new();
        
        self.registers
            .keys()
            .for_each(|k| register_vec.push((k.clone(), self.registers.get(k).unwrap().clone())));
    
        register_vec.sort_by(|(fst_key, _), (snd_key, _)| fst_key.cmp(snd_key));
        register_vec
    }

    /// Resets the contents of all registers to 0
    pub fn reset_registers(&mut self) {
        let mut registers: HashMap<u5,i32> = HashMap::new();
        for i in 0..31 as u8 {
            let index = u5::new(i);
            registers.insert(index, 0);
        }
        
        self.registers = registers;
    }

    #[allow(unused)]
    // debugging
    pub fn print_register_contents(&self) {
        self.registers
            .keys()
            .for_each(|k| println!("Register nr: {} Contents: {:?}", k, self.registers.get(k)));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ux::u5;
    #[test]
    fn test_get_registers_as_strings() {
        let register_file = RegisterFile::new();
        let registers = register_file.get_registers_and_values();
        
        for (reg,value) in registers {
            println!("{} , {}", reg,value);
        }
    }

    #[test]
    fn test_read_from_register(){
        let mut register_file = RegisterFile::new();
        assert_eq!(register_file.read_from_register(u5::new(8)), 0);
    }

    #[test]
    fn test_write_to_reg(){
        let mut register_file = RegisterFile::new();
        let t0: u5 = u5::new(8);
        register_file.write_to_register(t0, 5, true);

        assert_eq!(register_file.read_from_register(t0), 5);

        register_file.write_to_register(t0, 10, false);
        assert_eq!(register_file.read_from_register(t0), 5);
   }

   #[test]
   fn test_write_to_reg0(){
        let mut register_file = RegisterFile::new();
        register_file.write_to_register(u5::new(0), 50, true);
        assert_eq!(register_file.read_from_register(u5::new(0)), 0);
   }

   #[test]
   fn test_read_from_registers(){
        let mut register_file = RegisterFile::new();
        register_file.write_to_register(u5::new(1), 50, true);
        register_file.write_to_register(u5::new(2), 10, true);

        assert_eq!(register_file.read_from_registers(u5::new(1), u5::new(2)), (50,10));

   }

   #[test]
   fn reset_registers(){
        let mut register_file = RegisterFile::new();
        register_file.write_to_register(u5::new(1), 50, true);
        register_file.write_to_register(u5::new(2), 10, true);

        assert_eq!(register_file.read_from_registers(u5::new(1), u5::new(2)), (50,10));

        register_file.reset_registers();
        assert_eq!(register_file.read_from_registers(u5::new(1), u5::new(2)), (0,0));
   }

}
