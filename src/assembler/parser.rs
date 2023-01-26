//!  helper functions for parsing strings
//!  
//! File: parser.rs
//! Author: mai21asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0

/// begins_with_space: checks if the given instruction begins with a whitespace
///
/// input: instruction string
/// returns: true if begins with whitespace or false
///
pub fn begins_with_space(instruction: &str) -> bool {
    if instruction.len() == 0 {
        return false;
    }

    let char_vec: Vec<char> = instruction.chars().collect();
    char_vec[0].is_whitespace()
}

/// has_instruction:
/// Checks whether a given line contains an instruction.
/// Does not verify that the instruction in the line is a valid instruction.
/// input: the line to check if it contains an instruction
/// returns: a boolean representing whether the given line contains an instruction
pub fn has_instruction(line: &str) -> bool {
    if line.contains("#") {
        let maybe_without_comment = remove_comment_from_line(line);
        if maybe_without_comment.is_none() {
            return false;
        }

        let line = maybe_without_comment.unwrap();
        let split_str: Vec<&str> = line.split(":").collect();
        return split_str.len() == 1 || (split_str.len() > 1 && split_str[1].trim().len() > 0);
    }

    let split_str: Vec<&str> = line.trim().split(":").collect();

    split_str.len() == 1 && split_str[0].len() > 0
        || (split_str.len() > 1 && split_str[1].trim().len() > 0)
}

/// line_is_empty:
/// Checks if the given line is empty.
/// Is empty if it either has a length of 0 or consists of only whitespace.
/// input: the line to check if it is empty
/// returns: a bool representing whether the line is empty
pub fn line_is_empty(line: &str) -> bool {
    return line.len() == 0 || line.trim().is_empty();
}

/// remove_comment_from_line:
/// Returns the given line without a comment, or None if the comment
/// was the entire line.
/// If the given line does not contain a comment, the original line is returned.
/// input: reference to a string, not consumed after this function call
/// returns: new line without the comment or none
pub fn remove_comment_from_line(line: &str) -> Option<String> {
    let split_vec: Vec<&str> = line.split("#").collect();
    if split_vec[0].trim().to_string().len() == 0 {
        return None;
    }

    Some(split_vec[0].to_string())
}

/// get_comment_from_line:
/// Extracts the substring that is the comment from the line.
/// If the given line contains no comment, None is returned.
/// input: reference to the line
/// returns: a comment or none
#[allow(unused)]
pub fn get_comment_from_line(line: &str) -> Option<String> {
    let split_str: Vec<&str> = line.split("#").collect();
    if split_str.len() < 2 {
        return None;
    }
    let mut comment: String = String::from("");

    comment.push_str("#");
    comment.push_str(split_str[1]);
    Some(comment)
}

/// get_label_from_line:
/// Extracts the label from the given line.
/// If the given line contains no label, None is returned.
/// input: reference to the line
/// returns: a label or none
pub fn get_label_from_line(line: &str) -> Option<String> {
    let split_str: Vec<&str> = line.split(":").collect();
    if split_str.len() < 2 || split_str[0].contains("#") {
        return None;
    }

    Some(split_str[0].to_string())
}

/// remove_label_from_line:
///
/// input: reference to a string, not consumed after this function call
/// returns: new line without the label or none
pub fn remove_label_from_line(line: &str) -> Option<String> {
    let split_vec: Vec<&str> = line.split(":").collect();

    if split_vec.len() == 1 {
        // no label in line
        return Some(split_vec[0].to_string());
    }

    if split_vec[1].trim().to_string().len() == 0 {
        // only label in line..
        return None;
    }

    Some(split_vec[1].to_string())
}

/// extract_mnemonic_arguments_from_line:
///
/// input: &str
/// returns: string with mnemonics or none
#[allow(unused)]
pub fn extract_mnemonic_arguments_from_line(line: &str) -> Option<String> {
    let without_comment = remove_comment_from_line(line)?;
    let only_instr = remove_label_from_line(&without_comment)?;
    if only_instr.trim().len() > 0 {
        return Some(only_instr.trim().to_string());
    }

    None
}

/// trim_instruction_string: checks if the instruction starts with blank space and
///                           gets rid of that blank space.
///
/// input: string that expects no comment or label for now
/// returns: result vector.
#[allow(unused)]
pub fn trim_instruction_string(instruction: &str) -> Result<String, String> {
    if !begins_with_space(instruction) {
        // maybe not the best error message.......
        return Err(format!("{} does not start with blank space", instruction).to_string());
    }

    let split_vec: Vec<&str> = instruction.split(" ").collect();
    let mut trimmed_instr = String::new();
    trimmed_instr.push_str(split_vec[1]);

    let iterator = split_vec.iter();
    for text in iterator.skip(2) {
        trimmed_instr.push_str(" ");
        trimmed_instr.push_str(text);
    }

    Ok(trimmed_instr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
    #[test]
    fn test_remove_comment_from_line() {
        let line_with_comment = "I have a comment # this is my comment";
        let maybe_line = remove_comment_from_line(line_with_comment);
        assert!(maybe_line.is_some());
        let line = maybe_line.unwrap();
        assert_eq!(line, "I have a comment ");
    }

    #[test]
    fn test_get_comment_from_line() {
        let line_with_comment = "I have a comment # this is my comment";
        let maybe_line = get_comment_from_line(line_with_comment);
        assert!(maybe_line.is_some());
        let line = maybe_line.unwrap();
        assert_eq!(line, "# this is my comment");
    }

    #[test]
    fn test_remove_comment_without_comment() {
        let line_no_comment = "I have no comment";
        let maybe_line = remove_comment_from_line(line_no_comment);
        assert!(maybe_line.is_some());
        let line = maybe_line.unwrap();
        assert_eq!(line_no_comment, line);
    }

    #[test]
    fn test_get_label() {
        let line_with_label = "label: add $rd $rt $rs";
        let maybe_label = get_label_from_line(line_with_label);
        assert!(maybe_label.is_some());
        let label = maybe_label.unwrap();
        assert_eq!(label, "label");
    }

    #[test]
    fn test_get_label_only_label() {
        let only_label = "label:";
        let maybe_label = get_label_from_line(only_label);
        assert!(maybe_label.is_some());
        let label = maybe_label.unwrap();
        assert_eq!(label, "label");
    }

    #[test]
    fn test_get_label_no_label() {
        let no_label = "add $rd $rt $rs";
        let maybe_label = get_label_from_line(no_label);
        assert!(maybe_label.is_none());
    }

    #[test]
    fn test_has_instruction_label_comment() {
        let line_with_instruction = "label: addi $rd $rt 1 # good comment";
        assert!(has_instruction(line_with_instruction));
    }

    #[test]
    fn test_has_instruction_only_comment() {
        let comment = "#not an instruction";
        assert!(!has_instruction(comment));
    }

    #[test]
    fn test_has_instruction_only_label() {
        let label = "label:";
        assert!(!has_instruction(label));
    }

    #[test]
    fn test_has_instruction() {
        let instruction = "add $rd $rt $rs";
        assert!(has_instruction(instruction));
    }

    #[test]
    fn test_has_instruction_whitespace() {
        let whitespace = "      ";
        assert!(!has_instruction(whitespace));
    }

    #[test]
    fn test_remove_label_from_line() {
        let expected = " addi $sp, $sp, -8 #adjust stack pointer";
        let got = remove_label_from_line("fact: addi $sp, $sp, -8 #adjust stack pointer").unwrap();
        assert_eq!(expected, got);
    }
    #[test]
    fn test_trim_instruction_string_not_successful() {
        let faulty_instr = "addi  $t1, $zero, 1   # A comment";
        let result: Result<String, String> = trim_instruction_string(&faulty_instr);
        assert!(result.is_err());
    }

    #[test]
    fn test_trim_instruction_string_successful() {
        let expected = "addi $sp, $sp, -8 #adjust stack pointer";
        let got = trim_instruction_string(" addi $sp, $sp, -8 #adjust stack pointer").unwrap();
        assert_eq!(expected, got);
    }
    #[test]
    fn test_trim_instruction_string_misc() {
        let expected = "addi";
        let got = trim_instruction_string(" addi").unwrap();
        assert_eq!(expected, got);
    }
}
