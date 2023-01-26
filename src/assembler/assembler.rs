//!  This is a two-pass assembler, that gathers information
//!  about label values during first_pass through the source file and then using
//!  this information during the two_pass execution.
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0
use crate::assembler::first_pass;
use crate::assembler::second_pass;
use darken_assignment1::utils;
use self::utils::InstrRepresentable;

pub fn run_assembler(args: Vec<String>) -> Result<Vec<Box<dyn InstrRepresentable>>, String> {
    
    
    if args.len() < 1 {
        println!("Usage: 'cargo run path/to/input.txt path/to/instruction_output.txt path/to/listing_output.txt'");
        return Err("Usage: 'cargo run path/to/input.txt path/to/instruction_output.txt path/to/listing_output.txt'".to_string());
    } 
    let input_file_path = &args[1];
    let instruction_output_path;
    let listing_output_path;
    if args.len() < 3 {
        instruction_output_path = "default_instructions.txt";
        listing_output_path = "default_listings.txt";
    } else {
        instruction_output_path = &args[2];
        listing_output_path = &args[3];
    }
    

    if let Ok(lines) = utils::read_lines(input_file_path) {
        let table = first_pass::build_symbol_table(lines, &listing_output_path);
        
        let vec = second_pass::parse_write_instructions(
            input_file_path,
            instruction_output_path,
            listing_output_path,
            table,
        );

        if let Err(err_msg) = vec {
            return Err(err_msg);
        }

        
        vec
    } else {
        panic!("failed to read lines from input file {}", &args[1]);
    }

    
}

#[cfg(test)]
mod tests {
    use darken_assignment1::utils::read_lines;
    use std::fs::File;

    use super::*;

    fn check_instruction_output_from_input_file(
        input_path: &str,
        expected_output_path: &str,
        output_path: &str,
    ) {
        if let Ok(lines) = read_lines(input_path) {
            File::create(output_path).expect("Could not open output file!");
            let table = first_pass::build_symbol_table(lines, "");
            second_pass::parse_write_instructions(
                input_path,
                output_path,
                "test/test_listing_output.txt",
                table,
            ).unwrap();
            let expected_lines = read_lines(expected_output_path)
                .expect(format!("Could not open file {}", expected_output_path).as_str());
            let actual_lines = read_lines(output_path)
                .expect(format!("Could not open file {}", output_path).as_str());
            let line_pairings = expected_lines.zip(actual_lines);
            for (expected, actual) in line_pairings {
                assert!(expected.is_ok());
                assert!(actual.is_ok());
                assert_eq!(expected.unwrap(), actual.unwrap());
            }
        }
    }

    #[test]
    fn integration_test_1() {
        let input_path = "test/input1.txt";
        let expected_output_path = "test/expected_output1.txt";
        let output_path = "test/output1.txt";
        check_instruction_output_from_input_file(input_path, expected_output_path, output_path);
    }

    #[test]
    fn integration_test_2() {
        let input_path = "test/input2.txt";
        let expected_output_path = "test/expected_output2.txt";
        let output_path = "test/output2.txt";
        check_instruction_output_from_input_file(input_path, expected_output_path, output_path);
    }

    #[test]
    // This does not seem to pass when running all tests, but passes otherwise
    fn canvas_test_1() {
        let input_path = "test/canvas_test_1.txt";
        let expected_output_path = "test/canvas_output_instr_1.txt";
        let output_path = "test/test_output.txt";
        check_instruction_output_from_input_file(input_path, expected_output_path, output_path);
    }

    #[test]
    fn canvas_test_2() {
        let input_path = "test/canvas_test_2.txt";
        let expected_output_path = "test/canvas_output_instr_2.txt";
        let output_path = "test/test_canvas_output.txt";
        check_instruction_output_from_input_file(input_path, expected_output_path, output_path);
    }
}
