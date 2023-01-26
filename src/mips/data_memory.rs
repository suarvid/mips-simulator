//!  This file contains the datamemory of the MIPS processor. 
//! It is built as a vector of 1000 bytes, and each word is 4 bytes in this implementation.
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-25
//! Version: 1.0

pub struct DataMemory {
    contents: Vec<u8>,
}

impl DataMemory {
    /// Creates 1000 byte of byte-addressable
    /// memory, each byte initialized to 0.
    pub fn new() -> DataMemory {
        DataMemory {
            contents: vec![0; 1000],
        }
    }

    #[allow(unused)] // might be needed later
    pub fn get_byte(&self, idx: usize) -> Option<u8> {
        if idx >= self.contents.len() {
            return None;
        }

        Some(self.contents[idx])
    }

    pub fn get_word(&self, word_addr: usize, mem_read_sig: bool) -> Option<i32> {
        if mem_read_sig {
            if word_addr % 4 == 0 && word_addr < self.contents.len() {
                
                // all word addresses should be multiples of 4 I think
                let word = ((self.contents[word_addr] as u32) << 24)
                    + ((self.contents[word_addr + 1] as u32) << 16)
                    + ((self.contents[word_addr + 2] as u32) << 8)
                    + (self.contents[word_addr + 3] as u32);
                    
                
                return Some(word as i32);
            }
        }

        None
    }

    pub fn write_word_to_address(
        &mut self,
        address: usize,
        data: i32,
        mem_write_sig: bool,
    ) -> Result<(), &str> {
        if mem_write_sig {
            if address % 4 != 0 {
                return Err("Only allowed to write words on addresses that are multiples of 4.");
            }

            if address + 4 >= self.contents.len() {
                panic!("Trying to write outside of data memory!");
            }

            // have to split the data: u32 up into four bytes
            let bytes: [u8; 4] = data.to_be_bytes();

            self.contents[address] = bytes[0];
            self.contents[address + 1] = bytes[1];
            self.contents[address + 2] = bytes[2];
            self.contents[address + 3] = bytes[3];
            
        }

        Ok(())
    }

    pub fn get_contents(&self) -> Vec<(u32, u8)> {
        let mut v:Vec<(u32, u8)> = Vec::new();
        for (idx, val) in self.contents.iter().enumerate() {
            v.push((idx as u32, *val));
        }

        v
    }

    pub fn reset_contents(&mut self) {
        self.contents = vec![0; 1000];
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn test_write_word_to_address() {

        let mut data_mem = DataMemory::new();
        let write_res = data_mem.write_word_to_address(80, 1337, true);
        assert!(write_res.is_ok());
        let first = data_mem.get_byte(80).unwrap();
        let second = data_mem.get_byte(81).unwrap();
        let third = data_mem.get_byte(82).unwrap();
        let fourth = data_mem.get_byte(83).unwrap();
        assert_eq!(first, 0);
        assert_eq!(second, 0);
        assert_eq!(third, 5);
        assert_eq!(fourth, 57);

        let word = data_mem.get_word(80, true).unwrap();
        assert_eq!(word, 1337);
    }

    #[test]
    fn test_write_word_to_non_word_addr() {
        let mut data_mem = DataMemory::new();
        let write_res = data_mem.write_word_to_address(81, 123, true);
        assert!(write_res.is_err());
    }

    #[test]
    #[should_panic]
    fn test_write_word_last_byte_outside_mem() {
        let mut data_mem = DataMemory::new();
        let _write_res = data_mem.write_word_to_address(996, 123, true);
    }

    #[test]
    fn test_write_word_to_offset() {
        let mut data_mem = DataMemory::new();
        let write_res = data_mem.write_word_to_address(8, 200, true);
        assert!(write_res.is_ok());
        let read_res = data_mem.get_word(8, true).unwrap();
        assert_eq!(read_res, 200);
    }
}
