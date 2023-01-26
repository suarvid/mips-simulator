//! File with helper functions to the assembler.
//! File: utils.rs
//! Author: mai21asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0
#![allow(dead_code)]
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

/// read_lines:
/// input: filename
/// returns: an Iterator over the lines in the given file
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO: Move this to instruction or somewhere more sensible
/// Trait which represents that an object can be represented as a MIPS-instruction
pub trait InstrRepresentable {
    fn to_hex_string(&self) -> String;
    fn to_bin_string(&self) -> String;
    fn get_op(&self) -> String {
        return self
            .get_bits(31, 26)
            .expect("Every instruction should have an op-code in the 6 m.s.b.");
    }
    fn get_rs_str(&self) -> Option<String>;
    fn get_rt_str(&self) -> Option<String>;
    fn get_rd_str(&self) -> Option<String>;
    fn get_shamt_str(&self) -> Option<String>;
    fn get_funct_str(&self) -> Option<String>;
    fn get_imm_str(&self) -> Option<String>;
    fn get_jump_address_str(&self) -> Option<String>;
    fn to_mnemonic_string(&self) -> String;
    fn get_op_val(&self) -> i32;
    fn get_rs_val(&self) -> Option<i32>;
    fn get_rt_val(&self) -> Option<i32>;
    fn get_rd_val(&self) -> Option<i32>;
    fn get_shamt_val(&self) -> Option<i32>;
    fn get_funct_val(&self) -> Option<i32>;
    fn get_imm_val(&self) -> Option<i32>;
    fn get_jump_address_val(&self) -> Option<i32>;

    fn get_bits(&self, hi: usize, lo: usize) -> Option<String> {
        // need to check that hi is <= 31
        // and that low is >= 0
        // and also that low is not > hi
        if lo <= hi && hi <= 31 {
            // have to "flip" the given indexes?
            // i.e. bit 31 corresponds to index 0 in the string
            // 31 -> 0
            // 30 -> 1
            // 29 -> 2
            // 28 -> 3
            // 27 -> 4
            // X -> 31 - X it seems like
            let lo_flipped = 31 - lo;
            let hi_flipped = 31 - hi;
            let substr: String = self
                .to_bin_string()
                .chars()
                .skip(hi_flipped)
                .take((lo_flipped - hi_flipped) + 1)
                .collect();
            return Some(substr);
        }

        None
    }
}

/// print_to_file:
///
/// input: file_path - path to file
///        file_content - content to be written to file
/// returns: Result vector, err if can't print to file, Ok otherwise
pub fn print_to_file(file_path: &str, file_content: String) -> Result<(), ()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to open instruction output file");
    let write_res = file.write(file_content.as_bytes());

    if write_res.is_err() {
        return Err(());
    }

    Ok(())
}

/// helper function to flush contents of a file if necessary before a larger integration test.
pub fn flush_file(file_path: &str) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create(file_path)?);
    buffer.flush()?;
    Ok(())
}
