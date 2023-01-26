//! File for encoding instructions and saving them to an output-file + listings file with information
//!
//! File: second_pass.rs
//! Author: mai17asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0
use crate::assembler::instructions::parse_instruction;
use darken_assignment1::utils;
use darken_assignment1::utils::InstrRepresentable;
use std::collections::HashMap;
use std::fs::File;

use super::instructions::Instruction;

// TODO: get rid of this
type ErrorMessage = String;

/// Fills lines in output_line_instructions and output_line_listing Strings to be printed to file.
/// Parses instruction, produces hex-string representation of it, adds it to the instruction output.
/// Then generates a row to add to the listing file and adds that to the listing file output.
/// Returns: an error if parse_instruction fails due to wrong format, or none if succeeded
fn append_to_output(
    line: String,
    symbol_table: &HashMap<String, u32>,
    addr_counter: &mut u32,
    output_line_instructions: &mut String,
    output_line_listing: &mut String,
) -> Result<Option<Instruction>, ErrorMessage> {
    let hex_str: String;
    let addr_counter_str;
    let full_line = line.clone();
    let maybe_instruction = parse_instruction(line, &symbol_table, *addr_counter);
    if maybe_instruction.is_err() {
        return Err(maybe_instruction.unwrap_err());
    }

    if let Ok(Some(ref instruction)) = maybe_instruction {
        hex_str = instruction.to_hex_string();
        output_line_instructions.push_str(hex_str.as_str());
        output_line_instructions.push_str("\n");
        addr_counter_str = format!("{:#010x}", addr_counter);
        *addr_counter += 4;
    } else {
        hex_str = " ".repeat(10);
        addr_counter_str = " ".repeat(10);
    }

    output_line_listing
        .push_str(generate_row_for_listings(&addr_counter_str, &hex_str, &full_line).as_str());

    Ok(maybe_instruction.unwrap())
    // This is a lil ugly
    //Ok(maybe_instruction)
}

/// Reads lines from the given input file, tries to parse instructions
/// Uses the symbol table containing the address of each label in the
/// given input file in order to create proper instructions.
pub fn parse_write_instructions(
    input_file_path: &str,
    instruction_output_path: &str,
    assembly_listing_path: &str,
    symbol_table: HashMap<String, u32>,
) -> Result<Vec<Box< dyn InstrRepresentable>>, ErrorMessage> {
    File::create(instruction_output_path).expect(
        format!(
            "Failed to create instruction output file from path: {}.",
            instruction_output_path
        )
        .as_str(),
    );
    File::create(assembly_listing_path).expect(
        format!(
            "Failed to create assembly listing file from path: {}.",
            assembly_listing_path
        )
        .as_str(),
    );
    let lines = utils::read_lines(input_file_path).expect("Failed to open input file");
    let mut addr_counter = 0;
    let mut output_line_listing: String = String::new();
    let mut output_line_instructions: String = String::new();
    let mut instr_list: Vec<Box<dyn InstrRepresentable>> = Vec::new();

    for line in lines {
        if let Err(e) = line {
            utils::print_to_file(
                assembly_listing_path,
                format!("ERROR: Could not parse line from given input"),
            )
            .expect("Could not write to file");
            return Err(e.to_string());
        }

        if let Ok(line) = line {
            let res = append_to_output(
                line,
                &symbol_table,
                &mut addr_counter,
                &mut output_line_instructions,
                &mut output_line_listing,
            );
            if let Err(err) = res {
                output_line_listing.push_str(err.as_str());
                break;
            }
            if let Ok(Some(instr)) = res {
                instr_list.push(Box::new(instr));
            }
        }

    }

    output_line_listing.push_str("\n");
    // fill output line with the symbol table
    output_line_listing.push_str(generate_label_row(symbol_table).as_str());

    // print to files
    utils::print_to_file(assembly_listing_path, output_line_listing)
        .expect("Could not write to file");
    utils::print_to_file(instruction_output_path, output_line_instructions)
        .expect("Could not write to file");

    Ok(instr_list)
}

/// generates a row for symbols and returns as a string.
fn generate_label_row(symbols: HashMap<String, u32>) -> String {
    let mut output_line: String = String::new();
    output_line.push_str("Symbols\n");
    for (key, value) in symbols {
        output_line.push_str(key.as_str());
        output_line.push_str(" ".repeat(5).as_str());
        output_line.push_str(format!("{:#010x}", value).as_str());
        output_line.push_str(" ".repeat(10).as_str());
    }

    return output_line;
}

/// Creates a row to be printed to listings file
/// input: counter -
///        hex -
///        line -
/// Returns: string based on input, can be filled to another string that in turn can be printed later.
fn generate_row_for_listings(counter: &str, hex: &str, line: &str) -> String {
    let mut output_line: String = String::new();
    output_line.push_str(counter);
    output_line.push_str(" ".repeat(5).as_str());
    output_line.push_str(hex);
    output_line.push_str(" ".repeat(20).as_str());
    output_line.push_str(line);
    output_line.push_str("\n");

    return output_line;
}

#[cfg(test)]
mod tests {
    use darken_assignment1::utils::InstrRepresentable;

    use crate::assembler::instructions::{
        ITypeInstruction, JRTypeInstruction, JTypeInstruction, MemoryAccessTypeInstruction,
        RTypeInstruction,
    };

    #[test]
    fn test_hex_rep_addi() {
        let instr = ITypeInstruction::new("addi", "$zero", "$t1", "1", 0);
        assert!(instr.is_some());
        let mut hex_instr: u32 = 0;
        let instr = instr.unwrap();

        let op_val: u32 = instr.get_op().into();
        let rs_val: u32 = instr.get_rs().into();
        let rt_val: u32 = instr.get_rt().into();
        let imm_val = instr.get_imm() as u32;

        hex_instr += op_val << 26;
        hex_instr += rs_val << 21;
        hex_instr += rt_val << 16;
        hex_instr += imm_val;
        let actual = format!("{:#010x}", hex_instr);
        assert_eq!(actual, "0x20090001");
    }

    #[test]
    fn test_hex_rep_add() {
        let instr = RTypeInstruction::new("add", "$t1", "$t2", "$t0");
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.to_hex_string(), "0x012a4020");
    }

    #[test]
    fn test_hex_rep_sub() {
        let instr = RTypeInstruction::new("sub", "$t1", "$t2", "$t0");
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.to_hex_string(), "0x012a4022");
    }

    #[test]
    fn test_hex_rep_beq() {
        // Format of beq is: beq rs, rt, offset
        let instr = ITypeInstruction::beq("beq", "$t1", "$t2", "8", 0);
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.to_hex_string(), "0x112a0001"); // This will have to change probably
    }

    #[test]
    fn test_hex_rep_jump() {
        let instr = JTypeInstruction::new("j", "12");
        // because:
        // 12 = 0b1100
        // 0b1100 >> 2 = 0b0011 = 0x00000003
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.to_hex_string(), "0x08000003");
    }

    #[test]
    fn test_hex_rep_lw() {
        let instr = MemoryAccessTypeInstruction::new("lw", "$t0", "5", "$t1")
            .expect("Failed to create Memory Access Instruction");
        assert_eq!(instr.to_hex_string(), "0x8d280005");
    }

    #[test]
    fn test_hex_rep_sw() {
        let instr = MemoryAccessTypeInstruction::new("sw", "$t0", "5", "$t1")
            .expect("Failed to create Memory Access Instruction");
        assert_eq!(instr.to_hex_string(), "0xad280005");
    }

    #[test]
    fn test_hex_rep_jr() {
        let instr = JRTypeInstruction::new("jr", "$t0").expect("Failed to create JR instruction");
        assert_eq!(instr.to_hex_string(), "0x01000008");
    }

    #[test]
    fn test_hex_rep_sll() {
        let instr = RTypeInstruction::shift("sll", "$t0", "$t1", "2")
            .expect("Failed to create sll instruction"); // t0 = t1 << 2
        assert_eq!(instr.to_hex_string(), "0x00094080");
    }

    #[test]
    fn test_hex_rep_slt() {
        // slt rd rs rt, same as for add
        let instr = RTypeInstruction::new("slt", "$t1", "$t2", "$t0") // t0 = 1 if rs < rt
            .expect("Failed to create slt instruction");
        assert_eq!(instr.to_hex_string(), "0x012a402a");
    }

    #[test]
    fn test_hex_rep_and() {
        // and rd rs rt
        let instr = RTypeInstruction::new("and", "$t1", "$t2", "$t0")
            .expect("Failed to create and instrucion");
        assert_eq!(instr.to_hex_string(), "0x012a4024");
    }

    #[test]
    fn test_hex_rep_or() {
        let instr = RTypeInstruction::new("or", "$t1", "$t2", "$t0")
            .expect("Failed to create and instrucion");
        assert_eq!(instr.to_hex_string(), "0x012a4025");
    }

    #[test]
    fn test_hex_rep_nor() {
        let instr = RTypeInstruction::new("nor", "$t1", "$t2", "$t0")
            .expect("Failed to create nor instruction");
        assert_eq!(instr.to_hex_string(), "0x012a4027");
    }

    #[test]
    fn test_hex_rep_ori() {
        // ori rt, rs, imm
        let instr = ITypeInstruction::new("ori", "$t1", "$t0", "2", 0)
            .expect("Failed to create ori instruction");
        assert_eq!(instr.to_hex_string(), "0x35280002");
    }

    #[test]
    fn test_hex_rep_srl() {
        // srl rd, rt, sa
        let instr = RTypeInstruction::shift("srl", "$t0", "$t1", "2")
            .expect("Failed to create srl instruction");
        assert_eq!(instr.to_hex_string(), "0x00094082");
    }

    #[test]
    fn test_hex_rep_sra() {
        // sra t0, t1, 4 =  0x00094103
        let instr = RTypeInstruction::shift("sra", "$t0", "$t1", "4").unwrap();
        assert_eq!("0x00094103", instr.to_hex_string());
    }

    #[test]
    fn test_bin_rep_add() {
        let instr = RTypeInstruction::new("add", "$t1", "$t2", "$t0").unwrap();
        assert_eq!("00000001001010100100000000100000", instr.to_bin_string());
    }

    #[test]
    fn test_bin_rep_addi() {
        let instr = ITypeInstruction::new("addi", "$t1", "$t0", "1", 0).unwrap();
        assert_eq!("00100001001010000000000000000001", instr.to_bin_string())
    }

    #[test]
    fn test_bin_rep_jr() {
        let instr = JRTypeInstruction::new("jr", "$t0").expect("Failed to create JR instruction");
        assert_eq!("00000001000000000000000000001000", instr.to_bin_string());
    }

    #[test]
    fn test_bin_rep_lw() {
        let instr = MemoryAccessTypeInstruction::new("lw", "$t0", "5", "$t1").unwrap();
        assert_eq!("10001101001010000000000000000101", instr.to_bin_string());
    }

    #[test]
    fn test_bin_rep_j() {
        let instr = JTypeInstruction::new("j", "12").unwrap();
        assert_eq!("00001000000000000000000000000011", instr.to_bin_string());
    }
    #[test]
    fn test_hex_rep_lw_negative_offset() {
        let instr = MemoryAccessTypeInstruction::new("lw", "$t0", "-8", "$t1")
            .expect("Failed to create lw instruction!");
        assert_eq!(instr.to_hex_string(), "0x8d28fff8");
    }

    #[test]
    fn test_hex_rep_sw_negative_offset() {
        let instr = MemoryAccessTypeInstruction::new("sw", "$t0", "-8", "$t1")
            .expect("Failed to create sw instruction!");
        assert_eq!(instr.to_hex_string(), "0xad28fff8");
    }
}
// TODO: A bit weird that the hex_rep and bin_rep tests are placed here? When implementations are in instruction.rs?
