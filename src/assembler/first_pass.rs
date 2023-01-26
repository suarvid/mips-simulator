use crate::assembler::parser;
use darken_assignment1::utils::print_to_file;
/**
 * The first pass of the assembler will save all labels detected and return them in a symbol table
 *
* File: first_pass.rs
* Author: mai17asm, c19hln
* Since: 2022-11-24
* Version: 1.0
*
**/
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::process::exit;

/// builds and returns the table containing labels and their corresponding memory location.
///
/// input: lines - buffered reader with input.
/// returns: HashMap<String, u32> - the symbol table with labels as keys.
///
pub fn build_symbol_table(
    lines: Lines<BufReader<File>>,
    assembly_listing_path: &str,
) -> HashMap<String, u32> {
    let mut symbol_table = HashMap::new();
    let mut next_instruction = 0;

    for line in lines {
        if let Ok(line) = line {
            parser::line_is_empty("");

            if parser::line_is_empty(&line) {
                continue;
            }

            let maybe_line = parser::remove_comment_from_line(line.as_str());
            if maybe_line.is_none() {
                continue;
            }

            let line = maybe_line.unwrap();

            let maybe_label = parser::get_label_from_line(&line);
            if let Some(label) = maybe_label {
                if label.contains(" ") {
                    print_to_file(
                        assembly_listing_path,
                        "Found whitespace in label!".to_string(),
                    )
                    .expect("Could not write to file");
                    exit(1);
                }

                if symbol_table.contains_key(&label) {
                    print_to_file(assembly_listing_path, format!("Label {} is defined more than once in the given input file!", label).to_string())
                        .expect("Could not write to listings file");
                    exit(1);
                }

                symbol_table.insert(label, next_instruction);
            }

            if parser::has_instruction(&line) {
                next_instruction += 4;
            }
        } else {
            panic!("Error reading line from file!");
        }
    }

    symbol_table
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    /// file test/label_addr.txt seems to not exist :)
    #[allow(unused)]
    fn test_build_symbol_table() {
        let maybe_file = File::open("test/label_addr.txt");
        assert!(maybe_file.is_ok());
        let file = maybe_file.unwrap();

        let lines = BufReader::new(file).lines();
        let st = build_symbol_table(lines, "");
        assert!(st.contains_key("start"));
        assert!(st.contains_key("mid"));
        assert!(st.contains_key("end"));
        assert!(st.get("start").is_some());
        assert_eq!(st.get("start").unwrap().to_owned(), 0);
        assert_eq!(st.get("mid").unwrap().to_owned(), 8);
        assert_eq!(st.get("end").unwrap().to_owned(), 20);
    }
}
