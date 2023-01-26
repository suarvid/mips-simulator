use crate::assembler::parser::{has_instruction, remove_comment_from_line, remove_label_from_line};
use darken_assignment1::utils::InstrRepresentable;
/**
 * Implements the building of the supported instructions as well as the logic for parsing
 *  instructions with the help of functions from the parser.rs file
* File: instructions.rs
* Author: mai21asm, c19hln
* Since: 2022-11-24
* Version: 1.0
*
**/
use std::collections::HashMap;

use ux::{u10, u26, u5, u6};

// exit:
#[derive(Debug)]
pub struct TerminateInstruction {}
impl TerminateInstruction{
    pub fn to_hex_string(&self) -> String {
        "0xFFFFFFFF".to_string()
    }
}
impl InstrRepresentable for TerminateInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    fn to_bin_string(&self) -> String {
        "1".repeat(32)
    }

    fn get_rs_str(&self) -> Option<String> {
        self.get_bits(25, 21)
    }

    fn get_rt_str(&self) -> Option<String> {
        self.get_bits(20, 16)
    }

    fn get_rd_str(&self) -> Option<String> {
        self.get_bits(15, 11)
    }

    fn get_shamt_str(&self) -> Option<String> {
        self.get_bits(10, 6)
    }

    fn get_funct_str(&self) -> Option<String> {
        self.get_bits(5, 0)
    }

    fn get_imm_str(&self) -> Option<String> {
        self.get_bits(15, 0)
    }

    fn get_jump_address_str(&self) -> Option<String> {
        self.get_bits(25, 0)
    }

    fn to_mnemonic_string(&self) -> String {
        "exit".to_string()
    }

    fn get_op_val(&self) -> i32 {
        63 // 111111
    }

    fn get_rs_val(&self) -> Option<i32> {
        Some(31) // 11111
    }

    fn get_rt_val(&self) -> Option<i32> {
        Some(31) // 11111
    }
    fn get_rd_val(&self) -> Option<i32> {
        Some(31) // 11111
    }

    fn get_shamt_val(&self) -> Option<i32> {
        Some(31) // 11111
    }

    fn get_funct_val(&self) -> Option<i32> {
        Some(63) // 111111
    }

    fn get_imm_val(&self) -> Option<i32> {
        None
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        None
    }
}

#[derive(Debug)]
pub struct NopTypeInstruction {}
impl NopTypeInstruction {
    pub fn to_hex_string(&self) -> String {
        "0x00000000".to_string()
    }
}

impl InstrRepresentable for NopTypeInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    fn to_bin_string(&self) -> String {
        "0".repeat(32)
    }

    fn get_rs_str(&self) -> Option<String> {
        self.get_bits(25, 21)
    }

    fn get_rt_str(&self) -> Option<String> {
        self.get_bits(20, 16)
    }

    fn get_rd_str(&self) -> Option<String> {
        self.get_bits(15, 11)
    }

    fn get_shamt_str(&self) -> Option<String> {
        self.get_bits(10, 6)
    }

    fn get_funct_str(&self) -> Option<String> {
        self.get_bits(5, 0)
    }

    fn get_imm_str(&self) -> Option<String> {
        self.get_bits(15, 0)
    }

    fn get_jump_address_str(&self) -> Option<String> {
        self.get_bits(25, 0)
    }

    fn to_mnemonic_string(&self) -> String {
        "nop".to_string()
    }

    fn get_op_val(&self) -> i32 {
        0
    }

    fn get_rs_val(&self) -> Option<i32> {
        Some(0)
    }

    fn get_rt_val(&self) -> Option<i32> {
        Some(0)
    }
    fn get_rd_val(&self) -> Option<i32> {
        Some(0)
    }

    fn get_shamt_val(&self) -> Option<i32> {
        Some(0)
    }

    fn get_funct_val(&self) -> Option<i32> {
        Some(0)
    }

    fn get_imm_val(&self) -> Option<i32> {
        Some(0)
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        Some(0)
    }


}

#[derive(Debug)]
pub struct JRTypeInstruction {
    op: u6,
    rs: u5,
    zero: u10,
    hint: u5,
    func: u6,
}

impl JRTypeInstruction {
    pub fn new(instruction: &str, rs: &str) -> Option<JRTypeInstruction> {
        let num_op = get_numeric_op(instruction)?;
        let rs_num = get_register_number(rs)?;
        let func_num: u6 = get_func(instruction)?;

        Some(JRTypeInstruction {
            op: num_op,
            rs: rs_num,
            zero: u10::new(0),
            hint: u5::new(0),
            func: func_num,
        })
    }

    pub fn to_hex_string(&self) -> String {
        let mut hex_instr: u32 = 0;
        let rs_val: u32 = self.rs.into();
        let func_val: u32 = self.func.into();

        hex_instr += rs_val << 21;
        hex_instr += func_val;

        format!("{:#010x}", hex_instr).to_string()
    }

    pub fn to_bin_string(&self) -> String {
        let op_val = self.op;
        let rs_val = self.rs;
        let hint_val = self.hint;
        let func_val = self.func;
        let op_str = format!("{op_val:06b}");
        let rs_str = format!("{rs_val:05b}");
        let hint_str = format!("{hint_val:05b}");
        let func_str = format!("{func_val:06b}");
        let zeros_str = "0".repeat(10); //TODO: Make sure all JR instructions have 10 0's in the middle
        format!("{}{}{}{}{}", op_str, rs_str, zeros_str, hint_str, func_str)
    }
}

impl InstrRepresentable for JRTypeInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    // this is complex becasue attribute accesses aren't allowed in format!() for some reason
    fn to_bin_string(&self) -> String {
        self.to_bin_string()
    }

    fn get_rs_str(&self) -> Option<String> {
        self.get_bits(25, 21)
    }

    fn get_rt_str(&self) -> Option<String> {
        self.get_bits(20, 16)
    }

    fn get_rd_str(&self) -> Option<String> {
        self.get_bits(15, 11)
    }

    fn get_shamt_str(&self) -> Option<String> {
        self.get_bits(10, 6)
    }

    fn get_funct_str(&self) -> Option<String> {
        self.get_bits(5, 0)
    }

    fn get_imm_str(&self) -> Option<String> {
        None
    }

    fn get_jump_address_str(&self) -> Option<String> {
        None
    }
    fn to_mnemonic_string(&self) -> String {
        let mut mnemonic = String::new();
        mnemonic.push_str("jr ");
        let reg = get_register_name(self.rs).unwrap();
        mnemonic.push_str(&reg);

        mnemonic
    }

    fn get_op_val(&self) -> i32 {
        let op_u32: u32 = self.op.into();
        op_u32 as i32
    }

    fn get_rs_val(&self) -> Option<i32> {
        let rs_u32:u32 = self.rs.into();
        Some(rs_u32 as i32)
    }

    fn get_rt_val(&self) -> Option<i32> {
        None
    }
    
    fn get_rd_val(&self) -> Option<i32> {
        None
    }

    fn get_shamt_val(&self) -> Option<i32> {
        None
    }

    fn get_funct_val(&self) -> Option<i32> {
        None
    }

    fn get_imm_val(&self) -> Option<i32> {
        None
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        None
    }
}

#[derive(Debug)]
pub struct MemoryAccessTypeInstruction {
    op: u6,
    base: u5,
    rt: u5,
    offset: i16,
}

impl MemoryAccessTypeInstruction {
    pub fn new(
        instruction: &str,
        rt: &str,
        offset: &str,
        base: &str,
    ) -> Option<MemoryAccessTypeInstruction> {
        let num_op = get_numeric_op(instruction)?;
        let rt_num = get_register_number(rt)?;
        let base_num = get_register_number(base)?;
        let offset_num = i16::from_str_radix(offset, 10);
        if offset_num.is_err() {
            return None;
        }
        let offset_num = offset_num.unwrap();
        Some(MemoryAccessTypeInstruction {
            op: num_op,
            base: base_num,
            rt: rt_num,
            offset: offset_num,
        })
    }

    pub fn to_hex_string(&self) -> String {
        let mut hex_instr: u32 = 0;
        let op_val: u32 = self.op.into();
        let rt_val: u32 = self.rt.into();
        let base_val: u32 = self.base.into();
        hex_instr += op_val << 26;
        hex_instr += base_val << 21;
        hex_instr += rt_val << 16;
        hex_instr += (self.offset as u16) as u32;
        format!("{:#010x}", hex_instr).to_string()
    }

    pub fn to_bin_string(&self) -> String {
        // op, base, rt, offset
        let op_val = self.op;
        let base_val = self.base;
        let rt_val = self.rt;
        let offset_val = self.offset;

        let op_str = format!("{op_val:05b}");
        let base_str = format!("{base_val:05b}");
        let rt_str = format!("{rt_val:05b}");
        let offset_str = format!("{offset_val:016b}");

        format!("{}{}{}{}", op_str, base_str, rt_str, offset_str)
    }
}

impl InstrRepresentable for MemoryAccessTypeInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    fn to_bin_string(&self) -> String {
        self.to_bin_string()
    }

    fn get_rs_str(&self) -> Option<String> {
        self.get_bits(25, 21)
    }

    fn get_rt_str(&self) -> Option<String> {
        self.get_bits(20, 16)
    }

    fn get_rd_str(&self) -> Option<String> {
        None
    }

    fn get_shamt_str(&self) -> Option<String> {
        None
    }

    fn get_funct_str(&self) -> Option<String> {
        None
    }

    fn get_imm_str(&self) -> Option<String> {
        self.get_bits(15, 0)
    }

    fn get_jump_address_str(&self) -> Option<String> {
        None
    }

    fn to_mnemonic_string(&self) -> String {
        let mut mnemonic = String::new();
        let operation = get_operation(self.op, u6::new(0)).unwrap();
        let rt = get_register_name(self.rt).unwrap();
        let base = get_register_name(self.base).unwrap();
        mnemonic.push_str(&operation);
        mnemonic.push_str(" ");
        mnemonic.push_str(&rt);
        mnemonic.push_str(", ");
        mnemonic.push_str(&self.offset.to_string());
        mnemonic.push_str("(");
        mnemonic.push_str(&base);
        mnemonic.push_str(")");

        mnemonic
    }

    fn get_op_val(&self) -> i32 {
        let op_u32: u32 = self.op.into();
        op_u32 as i32
    }

    fn get_rs_val(&self) -> Option<i32> {
        let base: u32 = self.base.into();
        Some(base as i32)
    }

    fn get_rt_val(&self) -> Option<i32> {
        let rt: u32 = self.rt.into();
        Some(rt as i32) 
    }

    fn get_rd_val(&self) -> Option<i32> {
        None
    }

    fn get_shamt_val(&self) -> Option<i32> {
        None
    }

    fn get_funct_val(&self) -> Option<i32> {
        None
    }

    fn get_imm_val(&self) -> Option<i32> {
        Some(self.offset.into())
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        None
    }

}
#[derive(Debug)]

pub struct RTypeInstruction {
    op: u6,
    rs: u5,
    rt: u5,
    rd: u5,
    shamt: u5,
    func: u6,
}

impl RTypeInstruction {
    pub fn new(instruction: &str, rs: &str, rt: &str, rd: &str) -> Option<RTypeInstruction> {
        if instruction.contains("sll") || instruction.contains("sra") || instruction.contains("srl") {
            return RTypeInstruction::shift(instruction, rd, rs, rt);
        }
        let numeric_op = get_numeric_op(instruction)?;
        let rs_num = get_register_number(rs)?;
        let rt_num = get_register_number(rt)?;
        let rd_num = get_register_number(rd)?;
        let numeric_func = get_func(instruction)?;
        Some(RTypeInstruction {
            op: numeric_op,
            rs: rs_num,
            rt: rt_num,
            rd: rd_num,
            shamt: u5::new(0),
            func: numeric_func,
        })
    }

    pub fn shift(instruction: &str, rd: &str, rt: &str, shamt: &str) -> Option<RTypeInstruction> {
        let rd_num = get_register_number(rd)?;
        let rt_num = get_register_number(rt)?;
        let shamt_num = shamt.parse::<u8>().unwrap();

        Some(RTypeInstruction {
            op: u6::new(0),
            rs: u5::new(0),
            rt: rt_num,
            rd: rd_num,
            shamt: u5::new(shamt_num),
            func: get_func(instruction)?,
        })
    }


    pub fn to_hex_string(&self) -> String {
        let mut hex_instr: u32 = 0;
        let op_val: u32 = self.get_op().into();
        let rs_val: u32 = self.get_rs().into();
        let rt_val: u32 = self.get_rt().into();
        let rd_val: u32 = self.get_rd().into();
        let shamt_val: u32 = self.get_shamt().into();
        let func_val: u32 = self.get_func().into();
        hex_instr += op_val << 26;
        hex_instr += rs_val << 21;
        hex_instr += rt_val << 16;
        hex_instr += rd_val << 11;
        hex_instr += shamt_val << 6;
        hex_instr += func_val;
        format!("{:#010x}", hex_instr).to_string()
    }

    pub fn get_op(&self) -> u6 {
        self.op
    }

    pub fn get_rs(&self) -> u5 {
        self.rs
    }

    pub fn get_rt(&self) -> u5 {
        self.rt
    }

    pub fn get_rd(&self) -> u5 {
        self.rd
    }

    pub fn get_shamt(&self) -> u5 {
        self.shamt
    }

    pub fn get_func(&self) -> u6 {
        self.func
    }
}

impl InstrRepresentable for RTypeInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    fn to_bin_string(&self) -> String {
        let op_val = self.op;
        let rs_val = self.rs;
        let rt_val = self.rt;
        let rd_val = self.rd;
        let shamt_val = self.shamt;
        let func_val = self.func;
        let op_str = format!("{op_val:06b}");
        let rs_str = format!("{rs_val:05b}");
        let rt_str = format!("{rt_val:05b}");
        let rd_str = format!("{rd_val:05b}");
        let shamt_str = format!("{shamt_val:05b}");
        let func_str = format!("{func_val:06b}");
        format!("{}{}{}{}{}{}", op_str, rs_str, rt_str, rd_str, shamt_str, func_str)
    }

    fn get_rs_str(&self) -> Option<String> {
        self.get_bits(25, 21)
    }

    fn get_rt_str(&self) -> Option<String> {
        self.get_bits(20,16)
    }

    fn get_rd_str(&self) -> Option<String> {
        self.get_bits(15, 11)
    }

    fn get_shamt_str(&self) -> Option<String> {
        self.get_bits(10, 6)
    }

    fn get_funct_str(&self) -> Option<String> {
        self.get_bits(5, 0)
    }

    fn get_imm_str(&self) -> Option<String> {
        None
    }

    fn get_jump_address_str(&self) -> Option<String> {
        None
    }

    fn to_mnemonic_string(&self) -> String {
        let operation = get_operation(self.op, self.func).unwrap();
        let rs = get_register_name(self.rs).unwrap();
        let rt = get_register_name(self.rt).unwrap();
        let rd = get_register_name(self.rd).unwrap();
        let mut mnemonic = String::new();
        
        mnemonic.push_str(&operation);
        mnemonic.push_str(" ");
        mnemonic.push_str(&rd);
        mnemonic.push_str(", ");
        mnemonic.push_str(&rs);
        mnemonic.push_str(", ");
        mnemonic.push_str(&rt);


        mnemonic
        
    }

    fn get_op_val(&self) -> i32 {
        let op_val: u32 = self.op.into();
        op_val as i32
    }

    fn get_rs_val(&self) -> Option<i32> {
        let rs_val: u32 = self.rs.into();
        Some(rs_val as i32)
    }

    fn get_rt_val(&self) -> Option<i32> {
        let rt_val: u32 = self.rt.into();
        Some(rt_val as i32)
    }

    fn get_shamt_val(&self) -> Option<i32> {
        let shamt_val: u32 = self.shamt.into();
        Some(shamt_val as i32)
    }

    fn get_funct_val(&self) -> Option<i32> {
        let func_val: u32 = self.func.into();
        Some(func_val as i32)
    }

    fn get_imm_val(&self) -> Option<i32> {
        None
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        None
    }

    fn get_rd_val(&self) -> Option<i32> {
        let rd_val: u32 = self.rd.into();
        Some(rd_val as i32)
    }
}
    

#[derive(Debug)]
pub struct ITypeInstruction {
    op: u6,
    rs: u5,
    rt: u5,
    imm: i16,
}

impl ITypeInstruction {
    pub fn new(
        instruction: &str,
        rs: &str,
        rt: &str,
        imm: &str,
        current_addr: u32,
    ) -> Option<ITypeInstruction> {
        if instruction.contains("beq") {
            return ITypeInstruction::beq(instruction, rt, rs, imm, current_addr);
            // Switch these because MIPS is weird
        }
        let rt_num = get_register_number(rt)?;
        let rs_num = get_register_number(rs)?;
        let numeric_op = get_numeric_op(instruction)?;
        let imm_numeric = i16::from_str_radix(imm, 10);
        if imm_numeric.is_err() {
            return None;
        }
        Some(ITypeInstruction {
            op: numeric_op,
            rs: rs_num,
            rt: rt_num,
            imm: imm_numeric.unwrap(),
        })
    }

    pub fn beq(
        instruction: &str,
        rs: &str,
        rt: &str,
        imm: &str,
        current_addr: u32,
    ) -> Option<ITypeInstruction> {
        let rt_num = get_register_number(rt)?;
        let rs_num = get_register_number(rs)?;
        let numeric_op = get_numeric_op(instruction)?;
        let imm_numeric = i16::from_str_radix(imm, 10);
        if imm_numeric.is_err() {
            return None;
        }

        let offset: i16 = imm_numeric.unwrap() - (current_addr as i16 + 4);
        Some(ITypeInstruction {
            op: numeric_op,
            rs: rs_num,
            rt: rt_num,
            imm: offset,
        })
    }

    pub fn to_hex_string(&self) -> String {
        if self.op == get_numeric_op("beq").unwrap() {
            return self.to_hex_string_beq();
        }
        let mut hex_instr: u32 = 0;
        hex_instr += u32::from(self.get_op()) << 26;
        hex_instr += u32::from(self.get_rs()) << 21;
        hex_instr += u32::from(self.get_rt()) << 16;
        hex_instr += (self.imm as u16) as u32;
        format!("{:#010x}", hex_instr).to_string()
    }

    pub fn to_hex_string_beq(&self) -> String {
        let mut hex_instr: u32 = 0;
        hex_instr += u32::from(self.get_op()) << 26;
        hex_instr += u32::from(self.get_rs()) << 21;
        hex_instr += u32::from(self.get_rt()) << 16;
        // TODO: Might be something weird with overflowing shr here...
        if self.get_imm().is_negative() {
            // want to shift in 1's from the left
            let (shifted_imm, _) = self.get_imm().overflowing_shr(2);
            hex_instr += (shifted_imm as u16) as u32;
        } else {
            // want to shift in 0's from the left
            hex_instr += (self.get_imm() as u16) as u32 >> 2;
        }
        format!("{:#010x}", hex_instr).to_string()
    }

    pub fn to_bin_string(&self) -> String {
        // op, rs, rt, imm
        let op_val = self.op;
        let rs_val = self.rs;
        let rt_val = self.rt;
        let imm_val = self.imm;
        let op_str = format!("{op_val:06b}");
        let rs_str = format!("{rs_val:05b}");
        let rt_str = format!("{rt_val:05b}");
        let imm_str = format!("{imm_val:016b}");
        format!("{}{}{}{}", op_str, rs_str, rt_str, imm_str)
    }

    pub fn get_op(&self) -> u6 {
        self.op
    }

    pub fn get_rs(&self) -> u5 {
        self.rs
    }

    pub fn get_rt(&self) -> u5 {
        self.rt
    }

    pub fn get_imm(&self) -> i16 {
        self.imm
    }
}

impl InstrRepresentable for ITypeInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    fn to_bin_string(&self) -> String {
        self.to_bin_string()
    }

    fn get_rs_str(&self) -> Option<String> {
        self.get_bits(25, 21)
    }

    fn get_rt_str(&self) -> Option<String> {
        self.get_bits(20, 16)
    }

    fn get_imm_str(&self) -> Option<String> {
        self.get_bits(15, 0)
    }

    fn get_rd_str(&self) -> Option<String> {
        None
    }

    fn get_shamt_str(&self) -> Option<String> {
        None
    }

    fn get_funct_str(&self) -> Option<String> {
        None
    }

    fn get_jump_address_str(&self) -> Option<String> {
        None
    }

    fn to_mnemonic_string(&self) -> String {
        let operation = get_operation(self.op, u6::new(0)).unwrap();
        let rs = get_register_name(self.rs).unwrap();
        let rt = get_register_name(self.rt).unwrap();
        let mut mnemonic = String::new();
        
        mnemonic.push_str(&operation);
        mnemonic.push_str(" ");
        mnemonic.push_str(&rt);
        mnemonic.push_str(", ");
        mnemonic.push_str(&rs);
        mnemonic.push_str(", ");
        mnemonic.push_str(&self.imm.to_string());


        mnemonic
    }

    fn get_op_val(&self) -> i32 {
        let op_val: u32 = self.op.into();
        op_val as i32
    }

    fn get_rs_val(&self) -> Option<i32> {
        let rs_val: u32 = self.rs.into();
        Some(rs_val as i32)
    }

    fn get_rt_val(&self) -> Option<i32> {
        let rt_val: u32 = self.rt.into();
        Some(rt_val as i32)
    }

    fn get_shamt_val(&self) -> Option<i32> {
        None
    }

    fn get_funct_val(&self) -> Option<i32> {
        None
    }

    fn get_imm_val(&self) -> Option<i32> {
        Some(self.imm as i32)
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        None
    }

    fn get_rd_val(&self) -> Option<i32> {
        None
    }

}

#[derive(Debug)]
pub struct JTypeInstruction {
    op: u6,
    addr: u26,
}

impl JTypeInstruction {
    pub fn new(op: &str, target: &str) -> Option<JTypeInstruction> {
        let numeric_op = get_numeric_op(op).unwrap();
        let maybe_numeric_target = u32::from_str_radix(target, 10);
        //.expect(format!("Could not parse decimal value from given target {}", target).as_str());
        if let Ok(numeric_target) = maybe_numeric_target {
            return Some(JTypeInstruction {
                op: numeric_op,
                addr: u26::new(numeric_target),
            });
        }

        None
    }

    pub fn to_hex_string(&self) -> String {
        let mut hex_instr: u32 = 0;
        let op_val: u32 = self.get_op().into();
        let addr_val: u32 = self.get_addr().into();
        hex_instr += op_val << 26;
        // kanske ett bättre sätt att göra det på? ska alltid va 2 nollor i slutet pga word aligned..
        hex_instr += addr_val >> 2;

        format!("{:#010x}", hex_instr).to_string()
    }

    pub fn to_bin_string(&self) -> String {
        let op_val: u32 = self.op.into();
        let addr_val: u32 = self.addr.into();
        let mut instr_val: u32 = 0;
        instr_val += op_val << 26;
        instr_val += addr_val >> 2;
        format!("{instr_val:032b}")
    }

    pub fn get_op(&self) -> u6 {
        self.op
    }

    pub fn get_addr(&self) -> u26 {
        self.addr
    }

    
}

impl InstrRepresentable for JTypeInstruction {
    fn to_hex_string(&self) -> String {
        self.to_hex_string()
    }

    fn to_bin_string(&self) -> String {
        self.to_bin_string()
    }

    fn get_rs_str(&self) -> Option<String> {
        None
    }

    fn get_rt_str(&self) -> Option<String> {
        None
    }

    fn get_rd_str(&self) -> Option<String> {
        None
    }

    fn get_shamt_str(&self) -> Option<String> {
        None
    }

    fn get_funct_str(&self) -> Option<String> {
        None
    }

    fn get_imm_str(&self) -> Option<String> {
        None
    }

    fn get_jump_address_str(&self) -> Option<String> {
        self.get_bits(25, 0)
    }

    // will give you target in decimal form, not the label...
    fn to_mnemonic_string(&self) -> String {
        let mut mnemonic = String::new();
        mnemonic.push_str("j ");
        mnemonic.push_str(&self.addr.to_string());
        
        mnemonic
    }

    fn get_op_val(&self) -> i32 {
        let val: u32 = self.op.into();
        val as i32
    }

    fn get_rs_val(&self) -> Option<i32> {
        None
    }

    fn get_rt_val(&self) -> Option<i32> {
        None
    }

    fn get_shamt_val(&self) -> Option<i32> {
        None
    }

    fn get_funct_val(&self) -> Option<i32> {
        None
    }

    fn get_imm_val(&self) -> Option<i32> {
        None
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        let val: u32 = self.addr.into();
        Some(val as i32)
    }

    fn get_rd_val(&self) -> Option<i32> {
        None
    }

}

#[derive(Debug)]
pub enum Instruction {
    RType(RTypeInstruction),
    IType(ITypeInstruction),
    JType(JTypeInstruction),
    NopType(NopTypeInstruction),
    JRType(JRTypeInstruction),
    MemoryAccessType(MemoryAccessTypeInstruction),
    TerminateInstructionType(TerminateInstruction),
}

// is probably better to implement this for each type of instruction? Maybe?
impl InstrRepresentable for Instruction {
    fn to_hex_string(&self) -> String {
        match self {
            // Should be a better way to do this, right?
            Self::RType(instr) => instr.to_hex_string(),
            Self::IType(instr) => instr.to_hex_string(),
            Self::JType(instr) => instr.to_hex_string(),
            Self::NopType(instr) => instr.to_hex_string(),
            Self::JRType(instr) => instr.to_hex_string(),
            Self::MemoryAccessType(instr) => instr.to_hex_string(),
            Self::TerminateInstructionType(instr) => instr.to_hex_string(),
        }
    }

    fn to_bin_string(&self) -> String {
        match self {
            Self::RType(instr) => instr.to_bin_string(),
            Self::IType(instr) => instr.to_bin_string(),
            Self::JType(instr) => instr.to_bin_string(),
            Self::NopType(instr) => instr.to_bin_string(),
            Self::JRType(instr) => instr.to_bin_string(),
            Self::MemoryAccessType(instr) => instr.to_bin_string(),
            Self::TerminateInstructionType(instr) => instr.to_bin_string(),
        }
    }

    // TODO: Should Instruction even implement this trait? Is it ever used?
    fn get_rs_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_rs_str(),
            Self::IType(instr) => instr.get_rs_str(),
            Self::JType(instr) => instr.get_rs_str(),
            Self::NopType(instr) => instr.get_rs_str(),
            Self::JRType(instr) => instr.get_rs_str(),
            Self::MemoryAccessType(instr) => instr.get_rs_str(),
            Self::TerminateInstructionType(instr) => instr.get_rs_str(),
        }
    }

    fn get_rt_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_rt_str(),
            Self::IType(instr) => instr.get_rt_str(),
            Self::JType(instr) => instr.get_rt_str(),
            Self::NopType(instr) => instr.get_rt_str(),
            Self::JRType(instr) => instr.get_rt_str(),
            Self::MemoryAccessType(instr) => instr.get_rt_str(),
            Self::TerminateInstructionType(instr) => instr.get_rt_str(),
        }
    }

    fn get_rd_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_rd_str(),
            Self::IType(instr) => instr.get_rd_str(),
            Self::JType(instr) => instr.get_rd_str(),
            Self::NopType(instr) => instr.get_rd_str(),
            Self::JRType(instr) => instr.get_rd_str(),
            Self::MemoryAccessType(instr) => instr.get_rd_str(),
            Self::TerminateInstructionType(instr) => instr.get_rd_str(),
        }
    }

    fn get_shamt_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_shamt_str(),
            Self::IType(instr) => instr.get_shamt_str(),
            Self::JType(instr) => instr.get_shamt_str(),
            Self::NopType(instr) => instr.get_shamt_str(),
            Self::JRType(instr) => instr.get_shamt_str(),
            Self::MemoryAccessType(instr) => instr.get_shamt_str(),
            Self::TerminateInstructionType(instr) => instr.get_shamt_str(),
        }
    }

    fn get_funct_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_funct_str(),
            Self::IType(instr) => instr.get_funct_str(),
            Self::JType(instr) => instr.get_funct_str(),
            Self::NopType(instr) => instr.get_funct_str(),
            Self::JRType(instr) => instr.get_funct_str(),
            Self::MemoryAccessType(instr) => instr.get_funct_str(),
            Self::TerminateInstructionType(instr) => instr.get_funct_str(),
        }
    }

    fn get_imm_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_imm_str(),
            Self::IType(instr) => instr.get_imm_str(),
            Self::JType(instr) => instr.get_imm_str(),
            Self::NopType(instr) => instr.get_imm_str(),
            Self::JRType(instr) => instr.get_imm_str(),
            Self::MemoryAccessType(instr) => instr.get_imm_str(),
            Self::TerminateInstructionType(instr) => instr.get_imm_str(),
        }
    }

    fn get_jump_address_str(&self) -> Option<String> {
        match self {
            Self::RType(instr) => instr.get_jump_address_str(),
            Self::IType(instr) => instr.get_jump_address_str(),
            Self::JType(instr) => instr.get_jump_address_str(),
            Self::NopType(instr) => instr.get_jump_address_str(),
            Self::JRType(instr) => instr.get_jump_address_str(),
            Self::MemoryAccessType(instr) => instr.get_jump_address_str(),
            Self::TerminateInstructionType(instr) => instr.get_jump_address_str(),
        }
    }
    
    fn to_mnemonic_string(&self) -> String {
        match self {
            Self::RType(instr) => instr.to_mnemonic_string(),
            Self::IType(instr) => instr.to_mnemonic_string(),
            Self::JType(instr) => instr.to_mnemonic_string(),
            Self::NopType(instr) => instr.to_mnemonic_string(),
            Self::JRType(instr) => instr.to_mnemonic_string(),
            Self::MemoryAccessType(instr) => instr.to_mnemonic_string(),
            Self::TerminateInstructionType(instr) => instr.to_mnemonic_string(),
        }
    }

    fn get_op_val(&self) -> i32 {
        match self {
            Self::RType(instr) => instr.get_op_val(),
            Self::IType(instr) => instr.get_op_val(),
            Self::JType(instr) => instr.get_op_val(),
            Self::NopType(instr) => instr.get_op_val(),
            Self::JRType(instr) => instr.get_op_val(),
            Self::MemoryAccessType(instr) => instr.get_op_val(),
            Self::TerminateInstructionType(instr) => instr.get_op_val(),
        }
    }

    fn get_rs_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_rs_val(),
            Self::IType(instr) => instr.get_rs_val(),
            Self::JType(instr) => instr.get_rs_val(),
            Self::NopType(instr) => instr.get_rs_val(),
            Self::JRType(instr) => instr.get_rs_val(),
            Self::MemoryAccessType(instr) => instr.get_rs_val(),
            Self::TerminateInstructionType(instr) => instr.get_rs_val(),
        }
    }

    fn get_rt_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_rt_val(),
            Self::IType(instr) => instr.get_rt_val(),
            Self::JType(instr) => instr.get_rt_val(),
            Self::NopType(instr) => instr.get_rt_val(),
            Self::JRType(instr) => instr.get_rt_val(),
            Self::MemoryAccessType(instr) => instr.get_rt_val(),
            Self::TerminateInstructionType(instr) => instr.get_rt_val(),
        }
    }

    fn get_shamt_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_shamt_val(),
            Self::IType(instr) => instr.get_shamt_val(),
            Self::JType(instr) => instr.get_shamt_val(),
            Self::NopType(instr) => instr.get_shamt_val(),
            Self::JRType(instr) => instr.get_shamt_val(),
            Self::MemoryAccessType(instr) => instr.get_shamt_val(),
            Self::TerminateInstructionType(instr) => instr.get_shamt_val(),
        }
    }

    fn get_funct_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_funct_val(),
            Self::IType(instr) => instr.get_funct_val(),
            Self::JType(instr) => instr.get_funct_val(),
            Self::NopType(instr) => instr.get_funct_val(),
            Self::JRType(instr) => instr.get_funct_val(),
            Self::MemoryAccessType(instr) => instr.get_funct_val(),
            Self::TerminateInstructionType(instr) => instr.get_funct_val(),
        }
    }

    fn get_imm_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_imm_val(),
            Self::IType(instr) => instr.get_imm_val(),
            Self::JType(instr) => instr.get_imm_val(),
            Self::NopType(instr) => instr.get_imm_val(),
            Self::JRType(instr) => instr.get_imm_val(),
            Self::MemoryAccessType(instr) => instr.get_imm_val(),
            Self::TerminateInstructionType(instr) => instr.get_imm_val(),
        }
    }

    fn get_jump_address_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_jump_address_val(),
            Self::IType(instr) => instr.get_jump_address_val(),
            Self::JType(instr) => instr.get_jump_address_val(),
            Self::NopType(instr) => instr.get_jump_address_val(),
            Self::JRType(instr) => instr.get_jump_address_val(),
            Self::MemoryAccessType(instr) => instr.get_jump_address_val(),
            Self::TerminateInstructionType(instr) => instr.get_jump_address_val(),
        }
    }

    fn get_rd_val(&self) -> Option<i32> {
        match self {
            Self::RType(instr) => instr.get_rd_val(),
            Self::IType(instr) => instr.get_rd_val(),
            Self::JType(instr) => instr.get_rd_val(),
            Self::NopType(instr) => instr.get_rd_val(),
            Self::JRType(instr) => instr.get_rd_val(),
            Self::MemoryAccessType(instr) => instr.get_rd_val(),
            Self::TerminateInstructionType(instr) => instr.get_rd_val(),
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    RType,
    IType,
    JType,
    NopType,
    MemoryAccessType,
    JRType,
    TerminateInstructionType,
}

static VALID_INSTRUCTION: [&str; 18] = [
    "add", "sub", "and", "or", "nor", "slt", "sll", "jr", "nop", "lw", "sw", "beq", "addi", "j",
    "ori", "srl", "sra","exit",
];
static ZERO_OP: [&str; 11] = [
    "add", "sub", "and", "or", "nor", "slt", "sll", "jr", "nop", "sra", "srl",
];

/// parse_instruction: Parses the instruction represented on the line given as input.
///
/// input: line - string value of an instruction to be parsed
///        symbol_table - table with labels and corresponding memory locations.
///        current_addr - current address of program counter.
/// returns: instruction that was represented from the given line packeted in one of the above structures or none
///
pub fn parse_instruction(
    line: String,
    symbol_table: &HashMap<String, u32>,
    current_addr: u32,
) -> Result<Option<Instruction>, String> {
    if let Some(line) = remove_comment_and_label(line.as_str()) {
        if has_instruction(&line) {
            if contains_supported_instruction(&line) {
                let res =
                    parse_instruction_without_comment_label(&line, symbol_table, current_addr);
                if let Ok(instr) = res {
                    return Ok(Some(instr));
                }
                if let Err(err) = res {
                    return Err(err);
                }
            }
            return Err(
                format!("Could not parse instruction from given line {}", line).to_string(),
            );
        }
    }

    Ok(None)
}

/// get_instruction_type
///
/// input:
/// returns: JRType, NopType,JType,IType, MemoryAccessType or none
///
fn get_instruction_type(instruction: &str) -> Result<InstructionType, String> {
    match instruction {
        "nop" => Ok(InstructionType::NopType),
        "exit" => Ok(InstructionType::TerminateInstructionType),
        "srl" | "sra" | "add" | "sub" | "and" | "or" | "nor" | "slt" | "sll" => {
            Ok(InstructionType::RType)
        }
        "ori" | "addi" | "beq" => Ok(InstructionType::IType),
        "j" => Ok(InstructionType::JType),
        "jr" => Ok(InstructionType::JRType),
        "lw" | "sw" => Ok(InstructionType::MemoryAccessType),
        _ => Err(format!("{} is not a valid instruction", instruction).to_string()),
    }
}

/// get_arguments_instruction_type_instruction:
///
/// input:
/// returns:
///
fn get_arguments_instruction_type_instruction(
    line: &str,
) -> Result<(Vec<String>, InstructionType, &str), String> {
    let line_trimmed = line.trim();

    let maybe_instruction_delim = line_trimmed.find(" ");

    let instruction: &str;
    let argument_string: &str;

    if maybe_instruction_delim.is_none() {
        instruction = &line_trimmed[..];
        argument_string = "";
    } else {
        let instruction_delim = maybe_instruction_delim.unwrap();
        instruction = &line_trimmed[..instruction_delim];
        argument_string = &line_trimmed[instruction_delim..];
    }

    let instruction_type = get_instruction_type(instruction)?;

    let maybe_arguments = parse_arguments(argument_string, &instruction_type);
    if maybe_arguments.is_err() && instruction_type != InstructionType::NopType {
        return Err(format!(
            "Failed to parse arguments from string {} for an instruction of type {:?}",
            argument_string, instruction_type
        ));
    }

    let arguments = maybe_arguments.unwrap();

    Ok((arguments, instruction_type, instruction))
}

/// create_instruction:
///
/// input:
/// returns: Instruction of any of the types: JRType, NopType, JType, IType, MemoryAccessType
///
fn create_instruction(
    arguments: Vec<String>,
    instruction_type: &InstructionType,
    instruction: &str,
    symbol_table: &HashMap<String, u32>,
    current_addr: u32,
) -> Result<Instruction, String> {
    match instruction_type {
        InstructionType::NopType => Ok(Instruction::NopType(NopTypeInstruction {})),
        InstructionType::TerminateInstructionType=> Ok(Instruction::TerminateInstructionType(TerminateInstruction  {})),
        InstructionType::RType => {
            let maybe_instr = RTypeInstruction::new(
                instruction,
                &arguments[1].as_str(),
                &arguments[2].as_str(),
                &arguments[0].as_str(),
            );
            if let Some(instr) = maybe_instr {
                return Ok(Instruction::RType(instr));
            }
            Err(format!(
                "Could not create R-Type instruction from given string!
                Instruction: {}
                Arguments: {:#?}
                Current address: {}
                ",instruction, arguments, current_addr
            ))
        }
        InstructionType::IType => {
            let imm = &arguments[2];
            let maybe_instr = ITypeInstruction::new(
                instruction,
                &arguments[1].as_str(),
                &arguments[0].as_str(),
                &arguments[2].as_str(),
                current_addr,
            );
            if maybe_instr.is_none() {
                // imm could have been a label, not an actual value
                let maybe_addr = symbol_table.get(imm);
                if maybe_addr.is_none() {
                    return Err(format!(
                        "Could not find an address for label {} in the given symbol table.",
                        imm
                    ));
                }

                let maybe_instr = ITypeInstruction::new(
                    instruction,
                    &arguments[1].as_str(),
                    &arguments[0].as_str(),
                    maybe_addr.unwrap().to_string().as_str(),
                    current_addr,
                );

                if let Some(instr) = maybe_instr {
                    return Ok(Instruction::IType(instr));
                } else {
                    return Err(format!(
                        "Could not create I-Type instruction from given string!
                        Instruction: {}
                        Arguments: {:#?}
                        Current address: {}
                        ",instruction, arguments, current_addr
                    ))
                }
            } else {
                return Ok(Instruction::IType(maybe_instr.unwrap()));
            }
        }
        InstructionType::JType => {
            let target_label = &arguments[0];
            if let Some(target) = symbol_table.get(target_label) {
                let maybe_instr = JTypeInstruction::new(instruction, &target.to_string().as_str());
                if let Some(instr) = maybe_instr {
                    return Ok(Instruction::JType(instr));
                } else {
                    return Err(format!(
                        "Could not create J-Type instruction with op: {} and target: {}",
                        instruction, target
                    ));
                }
            } else {
                return Err(format!(
                    "Could not find address for label {} in symbol table",
                    target_label
                ));
            }
        }
        InstructionType::JRType => {
            let rs = &arguments[0];
            if let Some(instruction) = JRTypeInstruction::new(instruction, rs) {
                return Ok(Instruction::JRType(instruction));
            }
            return Err(format!("Could not create JR-Type instruction from line").to_string());
        }
        InstructionType::MemoryAccessType => {
            // lw rt, offset(base)
            let rt = &arguments[0];
            let offset = &arguments[1];
            let base = &arguments[2];

            let maybe_instr = MemoryAccessTypeInstruction::new(instruction, rt, offset, base);
            if maybe_instr.is_none() {
                return Err(format!(
                    "Could not create I-Type instruction from given string!
                    Instruction: {}
                    Arguments: {:#?}
                    Current address: {}
                    ",instruction, arguments, current_addr
                ))
            }

            return Ok(Instruction::MemoryAccessType(maybe_instr.unwrap()));
        }
        _ => Err("Unable to parse instruction parts into valid instruction.".to_string()),
    }
}

/// parse_instruction_without_comment_label:
///
/// input:
/// returns:
///
fn parse_instruction_without_comment_label(
    line: &str,
    symbol_table: &HashMap<String, u32>,
    current_addr: u32,
) -> Result<Instruction, String> {
    if let Ok((arguments, instruction_type, instruction)) =
        get_arguments_instruction_type_instruction(line)
    {
        return create_instruction(
            arguments,
            &instruction_type,
            instruction,
            symbol_table,
            current_addr,
        );
    }

    Err(format!(
        "Failed to parse arguments or instruction from line {}",
        line
    ))
}

/// get_expected_num_arguments: accesser for the number of arguments needed for a given instruction type.
///
/// input:
/// returns:
///
fn get_expected_num_arguments(instruction_type: &InstructionType) -> usize {
    match instruction_type {
        InstructionType::RType | InstructionType::IType | InstructionType::MemoryAccessType => 3,
        InstructionType::JType | InstructionType::JRType => 1,
        InstructionType::NopType | InstructionType::TerminateInstructionType => 0,
    }
}

/// parse_memory_access_arguments: Parses the arguments for the instruction type MemoryAccessType
///
/// input:
/// returns:
///
fn parse_memory_access_arguments(argument_string: &str) -> Result<Vec<String>, String> {
    let mut arg_vec = Vec::new();

    argument_string
        .split(',')
        .for_each(|arg| arg_vec.push(arg.trim().to_string()));

    if arg_vec.len() < 2 {
        return Err(
            "Argument string did not contain comma, expects comma separating arguments".to_string(),
        );
    }

    if !arg_vec[1].contains('(') {
        return Err("Memory access instruction does not contain opening parentheses".to_string());
    }

    let offset_base: Vec<&str> = arg_vec[1].split('(').collect();
    let untrimmed_offset = offset_base[0];
    let base = offset_base[1];

    let end_idx = base.find(')');
    if end_idx.is_none() {
        return Err("Could not find closing parenthesis in JR instruction".to_string());
    }

    let end_idx = end_idx.unwrap();

    let (untrimmed_base, _) = base.split_at(end_idx);
    let expected_num_args = get_expected_num_arguments(&InstructionType::MemoryAccessType);

    let return_vec = vec![
        arg_vec[0].to_string(),
        untrimmed_offset.trim().to_string(),
        untrimmed_base.trim().to_string(),
    ];

    if get_expected_num_arguments(&InstructionType::MemoryAccessType) != return_vec.len() {
        return Err(format!(
            "Expected {} arguments, got {}",
            expected_num_args,
            arg_vec.len()
        )
        .to_string());
    }

    Ok(return_vec)
}

/// parse_jr_arguments: Parses the arguments for the instruction type JR
///
/// input:
/// returns:
///
fn parse_jr_arguments(argument_string: &str) -> Result<Vec<String>, String> {
    let mut arg_vec = Vec::new();
    argument_string
        .split(',')
        .for_each(|arg| arg_vec.push(arg.trim().to_string()));
    let num_args = arg_vec.len();
    let expected_num_args = get_expected_num_arguments(&InstructionType::JRType);
    if num_args == expected_num_args {
        return Ok(vec![arg_vec[0].trim().to_string()]);
    }

    println!(
        "Expected_num_args: {}, num_args: {}",
        expected_num_args, num_args
    );

    Err(format!(
        "Expected {} argument(s), got {}.",
        expected_num_args, num_args
    ))
}

/// parse_arguments:
///
/// input: argument_string - string with no comment or label
///        instruction_type - JType, JRType, IType, MemoryAccessType,
/// returns: a string-vector with the arguments depending on type of instruction.
///
fn parse_arguments(
    argument_string: &str,
    instruction_type: &InstructionType,
) -> Result<Vec<String>, String> {
    return match instruction_type {
        InstructionType::MemoryAccessType => parse_memory_access_arguments(argument_string),
        InstructionType::JRType => parse_jr_arguments(argument_string),
        _ => {
            let mut arg_vec = Vec::new();
            argument_string.split(',').for_each(|arg| {
                if !arg.trim().to_string().is_empty() {
                    arg_vec.push(arg.trim().to_string())
                }
            });

            let expected_num_args = get_expected_num_arguments(instruction_type);

            if arg_vec.len() != expected_num_args {
                return Err(format!(
                    "Expected {} arguments, got {}, instruction type: {:?}, argument string: {}",
                    expected_num_args,
                    arg_vec.len(),
                    instruction_type,
                    argument_string
                ));
            }

            Ok(arg_vec)
        }
    };
}

/// contains_supported_instruction: checks if the instruction is supported by the assembler.
///
/// input: full instruction as a string reference
/// returns: true if supported instruction or false
///
fn contains_supported_instruction(instruction: &str) -> bool {
    let split_str: Vec<&str> = instruction.trim().split(" ").collect();
    VALID_INSTRUCTION.contains(&split_str[0])
}

/// get_func: getter for the func-value of an instruction string if such one exist for the instruction
///
/// input: &str
/// returns: func associated with instruction or none
///
pub fn get_func(instruction: &str) -> Option<u6> {
    let maybe_val = match instruction {
        "add" => Some(32),
        "sub" => Some(34),
        "and" => Some(36),
        "or" => Some(37),
        "nor" => Some(39),
        "slt" => Some(42),
        "sll" => Some(0),
        "jr" => Some(8),
        "nop" => Some(0),
        "sra" => Some(3),
        "srl" => Some(2),
        _ => None,
    };

    if let Some(val) = maybe_val {
        return Some(u6::new(val));
    }

    None
}

/// get_numeric_op:
///
/// input: &str instruction line
/// returns: the op code from the instruction, none if instruction is not supported
///
pub fn get_numeric_op(instruction: &str) -> Option<u6> {
    if ZERO_OP.contains(&instruction) {
        return Some(u6::new(0));
    }

    let maybe_val = match instruction {
        "lw" => Some(35),
        "sw" => Some(43),
        "beq" => Some(4),
        "addi" => Some(8),
        "ori" => Some(13),
        "j" => Some(2),
        _ => None,
    };

    if let Some(val) = maybe_val {
        return Some(u6::new(val));
    }

    None
}

/// get_register_name: Maps a register number tot a register name
///
/// input: register_number: u5 representing a number 0-31 
/// returns: the register name, none if invalid register number
///
pub fn get_register_name(register_number: u5) -> Option<String> {
    let reg_num_val: u8 = register_number.into();
    match reg_num_val {
        0 => return Some("$zero".to_string()),
        1 => return Some("$at".to_string()),
        2 => return Some("$v0".to_string()),
        3 => return Some("$v1".to_string()),
        4 => return Some("$a0".to_string()),
        5 => return Some("$a1".to_string()),
        6 => return Some("$a2".to_string()),
        7 => return Some("$a3".to_string()),
        8 => return Some("$t0".to_string()),
        9 => return Some("$t1".to_string()),
        10 => return Some("$t2".to_string()),
        11 => return Some("$t3".to_string()),
        12 => return Some("$t4".to_string()),
        13 => return Some("$t5".to_string()),
        14 => return Some("$t6".to_string()),
        15 => return Some("$t7".to_string()),
        16 => return Some("$s0".to_string()),
        17 => return Some("$s1".to_string()),
        18 => return Some("$s2".to_string()),
        19 => return Some("$s3".to_string()),
        20 => return Some("$s4".to_string()),
        21 => return Some("$s5".to_string()),
        22 => return Some("$s6".to_string()),
        23 => return Some("$s7".to_string()),
        24 => return Some("$t8".to_string()),
        25 => return Some("$t9".to_string()),
        26 => return Some("$k0".to_string()),
        27 => return Some("$k1".to_string()),
        28 => return Some("$gp".to_string()),
        29 => return Some("$sp".to_string()),
        30 => return Some("$fp".to_string()),
        31 => return Some("$ra".to_string()),
        _=> return None,

    }
}
/// get_operation: Maps op + func to a supported instruction
///
/// input: op: u6 (op code), func: u6 func code
/// returns: the name of the operation supported, or none if not supported in the assembler
///
fn get_operation(op: u6, func: u6) -> Option<String>{
    if op == u6::new(0) {
        
        return match u8::from(func) {
            32 => Some("add".to_string()),
            34 => Some("sub".to_string()),
            36 => Some("and".to_string()),
            37 => Some("or".to_string()),
            39 => Some("nor".to_string()),
            42 => Some("slt".to_string()),
            0 => Some("sll".to_string()),
            8 => Some("jr".to_string()),
            3 => Some("sra".to_string()),
            2 => Some("srl".to_string()),
            _=> None,
        }
    }


    match u8::from(op) {
        35 => Some("lw".to_string()),
        43 => Some("sw".to_string()),
        4 => Some("beq".to_string()),
        8 => Some("addi".to_string()),
        2 => Some("j".to_string()),
        13 => Some("ori".to_string()),
        _=> None, 
    }

    

    
}

/// get_register_number:
///
/// input: string that starts with $
/// returns: the number representing the given register in MIPS, None if an invalid register name is given
///
pub fn get_register_number(register: &str) -> Option<u5> {
    let ascii_lowercase = register.to_ascii_lowercase();
    let chars: Vec<char> = ascii_lowercase.chars().collect();

    if chars.len() < 3 {
        return None;
    }

    let reg_num: i32 = match register {
        "$zero" => 0,
        "$at" => 1,
        "$a0" => 4,
        "$a1" => 5,
        "$a2" => 6,
        "$a3" => 7,
        "$gp" => 28,
        "$sp" => 29,
        "$fp" => 30,
        "$ra" => 31,

        _ => {
            let spec_reg_nr = chars[2];
            match chars[1] {
                's' => to_decimal_digit(spec_reg_nr)? as i32 + 16,
                't' => {
                    if (to_decimal_digit(spec_reg_nr)? as i32) < 8 {
                        to_decimal_digit(spec_reg_nr)? as i32 + 8
                    } else {
                        to_decimal_digit(spec_reg_nr)? as i32 + 16
                    }
                }
                'k' => to_decimal_digit(spec_reg_nr)? as i32 + 26,
                'v' => to_decimal_digit(spec_reg_nr)? as i32 + 2,
                _ => -1,
            }
        }
    };

    if reg_num < 0 {
        return None;
    }

    Some(u5::new(reg_num as u8))
}

/// to_decimal_digit: Parses the character at the given index into a decimal digit
/// input: character to convert
/// returns: converted character to 32-bit integer
///
fn to_decimal_digit(to_convert: char) -> Option<u32> {
    // was -> i32
    to_convert.to_digit(10)
}

///
/// remove_comment_and_label:
///
/// input: line - reference to a string
/// output: An option, either none if the line is empty or a new string value without the comment and label
///
fn remove_comment_and_label(line: &str) -> Option<String> {
    if let Some(line) = remove_comment_from_line(&line) {
        if let Some(line) = remove_label_from_line(&line) {
            return Some(line.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_ascii() {
        let string = "$t0";
        let ascii_lowercase = string.to_ascii_lowercase();
        let _test: Vec<char> = ascii_lowercase.chars().collect();
        
    }
    #[test]
    fn test_get_register_name(){
        /* misc registers:  */
        let number = get_register_number("$zero").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$zero");
        let number = get_register_number("$at").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$at");
        let number = get_register_number("$gp").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$gp");
        let number = get_register_number("$sp").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$sp");
        let number = get_register_number("$fp").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$fp");
        let number = get_register_number("$ra").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$ra");

        /* v-registers */ 
        let number = get_register_number("$v0").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$v0");
        let number = get_register_number("$v1").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$v1");
        
        /* a-registers */ 
        let number = get_register_number("$a0").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$a0");
        let number = get_register_number("$a1").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$a1");
        let number = get_register_number("$a2").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$a2");
        let number = get_register_number("$a3").unwrap();
        let name = get_register_name(number).unwrap();
        assert_eq!(name, "$a3");
        
        /* t-registers */ 
        let number = get_register_number("$t0").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t0");
        let number = get_register_number("$t1").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t1");
        let number = get_register_number("$t2").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t2");
        let number = get_register_number("$t3").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t3");
        let number = get_register_number("$t4").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t4");
        let number = get_register_number("$t5").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t5");
        let number = get_register_number("$t6").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t6");
        let number = get_register_number("$t7").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t7");
        let number = get_register_number("$t8").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t8");
        let number = get_register_number("$t9").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$t9");
        
        /* s-registers */ 
        let number = get_register_number("$s0").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s0");
        let number = get_register_number("$s1").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s1");
        let number = get_register_number("$s2").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s2");
        let number = get_register_number("$s3").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s3");
        let number = get_register_number("$s4").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s4");
        let number = get_register_number("$s5").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s5");
        let number = get_register_number("$s6").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s6");
        let number = get_register_number("$s7").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$s7");

        /* k-registers */ 
        let number = get_register_number("$k0").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$k0");
        let number = get_register_number("$k1").unwrap();
        assert_eq!(get_register_name(number).unwrap(), "$k1");
        
    }

    #[test] // Test från helvetet
    fn test_reg_numbers() {
        // Test t-registers
        let maybe_regnr = get_register_number("$t0");
        assert_eq!(maybe_regnr.unwrap(), u5::new(8));
        let maybe_regnr = get_register_number("$t1");
        assert_eq!(maybe_regnr.unwrap(), u5::new(9));
        let maybe_regnr = get_register_number("$t2");
        assert_eq!(maybe_regnr.unwrap(), u5::new(10));
        let maybe_regnr = get_register_number("$t3");
        assert_eq!(maybe_regnr.unwrap(), u5::new(11));
        let maybe_regnr = get_register_number("$t4");
        assert_eq!(maybe_regnr.unwrap(), u5::new(12));
        let maybe_regnr = get_register_number("$t5");
        assert_eq!(maybe_regnr.unwrap(), u5::new(13));
        let maybe_regnr = get_register_number("$t6");
        assert_eq!(maybe_regnr.unwrap(), u5::new(14));
        let maybe_regnr = get_register_number("$t7");
        assert_eq!(maybe_regnr.unwrap(), u5::new(15));
        let maybe_regnr = get_register_number("$t8");
        assert_eq!(maybe_regnr.unwrap(), u5::new(24));
        let maybe_regnr = get_register_number("$t9");
        assert_eq!(maybe_regnr.unwrap(), u5::new(25));

        // Test s-registers
        let maybe_regnr = get_register_number("$s0");
        assert_eq!(maybe_regnr.unwrap(), u5::new(16));
        let maybe_regnr = get_register_number("$s1");
        assert_eq!(maybe_regnr.unwrap(), u5::new(17));
        let maybe_regnr = get_register_number("$s2");
        assert_eq!(maybe_regnr.unwrap(), u5::new(18));
        let maybe_regnr = get_register_number("$s3");
        assert_eq!(maybe_regnr.unwrap(), u5::new(19));
        let maybe_regnr = get_register_number("$s4");
        assert_eq!(maybe_regnr.unwrap(), u5::new(20));
        let maybe_regnr = get_register_number("$s5");
        assert_eq!(maybe_regnr.unwrap(), u5::new(21));
        let maybe_regnr = get_register_number("$s6");
        assert_eq!(maybe_regnr.unwrap(), u5::new(22));
        let maybe_regnr = get_register_number("$s7");
        assert_eq!(maybe_regnr.unwrap(), u5::new(23));

        // Test argument registers
        let maybe_regnr = get_register_number("$a0");
        assert_eq!(maybe_regnr.unwrap(), u5::new(4));
        let maybe_regnr = get_register_number("$a1");
        assert_eq!(maybe_regnr.unwrap(), u5::new(5));
        let maybe_regnr = get_register_number("$a2");
        assert_eq!(maybe_regnr.unwrap(), u5::new(6));
        let maybe_regnr = get_register_number("$a3");
        assert_eq!(maybe_regnr.unwrap(), u5::new(7));

        // Test value registers
        let maybe_regnr = get_register_number("$v0");
        assert_eq!(maybe_regnr.unwrap(), u5::new(2));
        let maybe_regnr = get_register_number("$v1");
        assert_eq!(maybe_regnr.unwrap(), u5::new(3));

        // Test Kernel registers
        let maybe_regnr = get_register_number("$k0");
        assert_eq!(maybe_regnr.unwrap(), u5::new(26));
        let maybe_regnr = get_register_number("$k1");
        assert_eq!(maybe_regnr.unwrap(), u5::new(27));

        // Test misc registers
        let maybe_regnr = get_register_number("$zero");
        assert_eq!(maybe_regnr.unwrap(), u5::new(0));
        let maybe_regnr = get_register_number("$at");
        assert_eq!(maybe_regnr.unwrap(), u5::new(1));
        let maybe_regnr = get_register_number("$gp");
        assert_eq!(maybe_regnr.unwrap(), u5::new(28));
        let maybe_regnr = get_register_number("$sp");
        assert_eq!(maybe_regnr.unwrap(), u5::new(29));
        let maybe_regnr = get_register_number("$fp");
        assert_eq!(maybe_regnr.unwrap(), u5::new(30));
        let maybe_regnr = get_register_number("$ra");
        assert_eq!(maybe_regnr.unwrap(), u5::new(31));
    }

    #[test]
    fn test_contains_supported_instruction() {
        assert!(contains_supported_instruction(
            "add $t1, $2, $t3    # A comment"
        ));
        assert!(contains_supported_instruction("sub"));
        assert!(contains_supported_instruction("and"));
        assert!(contains_supported_instruction("or"));
        assert!(contains_supported_instruction("nor"));
        assert!(contains_supported_instruction("slt"));
        assert!(contains_supported_instruction("lw"));
        assert!(contains_supported_instruction("sw"));
        assert!(contains_supported_instruction("beq"));
        assert!(contains_supported_instruction(
            "addi  $t1, $zero, 1   # A comment"
        ));
        assert!(contains_supported_instruction("sll"));
        assert!(contains_supported_instruction("j"));
        assert!(contains_supported_instruction("jr"));
        assert!(contains_supported_instruction("nop"));
    }

    #[test]
    fn test_parse_rtype_arguments_successful() {
        let arg_str = "$t0, $t1, $t2";
        let instr_type = InstructionType::RType;
        let parse_result = parse_arguments(arg_str, &instr_type);
        assert!(parse_result.is_ok());
        let parsed_args = parse_result.unwrap();
        assert_eq!(parsed_args[0], "$t0");
        assert_eq!(parsed_args[1], "$t1");
        assert_eq!(parsed_args[2], "$t2");
    }

    #[test]
    fn test_parse_rtype_arguments_unsuccessful() {
        let arg_str = "$t0, $t1";
        let instr_type = InstructionType::RType;
        let parse_result = parse_arguments(arg_str, &instr_type);
        assert!(parse_result.is_err());
    }

    #[test]
    fn test_parse_itype_arguments_successful() {
        let arg_str = "$t0, $t1, 5";
        let instr_type = InstructionType::IType;
        let parse_result = parse_arguments(arg_str, &instr_type);
        assert!(parse_result.is_ok());
        let parsed_args = parse_result.unwrap();
        assert_eq!("$t0", parsed_args[0]);
        assert_eq!("$t1", parsed_args[1]);
        assert_eq!("5", parsed_args[2]);
    }

    #[test]
    fn test_parse_itype_arguments_unsuccessful() {
        let arg_str = "$t0, $t1";
        let instr_type = InstructionType::IType;
        let parse_result = parse_arguments(arg_str, &instr_type);
        assert!(parse_result.is_err());
    }

    // Probably want to add one test for each supported instruction
    #[test]
    fn test_create_add_instruction() {
        let rs = "$t0";
        let rt = "$t1";
        let rd = "$t2";
        let instr = "add";
        let maybe_instr = RTypeInstruction::new(instr, rs, rt, rd); // add $t2, $t0, $t1
        assert!(maybe_instr.is_some());
        let add_instr = maybe_instr.unwrap();
        assert_eq!(add_instr.op, get_numeric_op(instr).unwrap());
        assert_eq!(add_instr.rs, get_register_number(rs).unwrap());
        assert_eq!(add_instr.rt, get_register_number(rt).unwrap());
        assert_eq!(add_instr.rd, get_register_number(rd).unwrap());
        assert_eq!(add_instr.shamt, u5::new(0));
    }

    #[test]
    fn test_create_addi_instruction() {
        let instr = "addi";
        let rs = "$t0";
        let rt = "$t1";
        let imm = "15";
        let addi_instr = ITypeInstruction::new(instr, rs, rt, imm, 0); // rs, rt, imm
        assert!(addi_instr.is_some());
        let addi_instr = addi_instr.unwrap();
        assert_eq!(addi_instr.op, get_numeric_op(instr).unwrap());
        assert_eq!(addi_instr.imm, 15);
        assert_eq!(addi_instr.rs, get_register_number(rs).unwrap());
        assert_eq!(addi_instr.rt, get_register_number(rt).unwrap());
    }

    #[test]
    fn test_create_jmp_instruction() {
        let instr = "j";
        let target = "40";
        let jmp_instr = JTypeInstruction::new(instr, target).unwrap();
        assert_eq!(get_numeric_op(instr).unwrap(), jmp_instr.op);
        // detta är ej snyggt I know, måste fråga tomas/oskar
        assert_eq!(jmp_instr.addr, u26::new(40));
    }
    #[test]
    fn test_create_sll_instruction() {
        let rt = "$t1";
        let rd = "$t2";
        let instr = "sll";
        let shamt = "4";
        let sll_instr = RTypeInstruction::shift(instr, rd, rt, shamt).unwrap(); // add $t2, $t0, $t1
        assert_eq!(sll_instr.op, get_numeric_op(instr).unwrap());
        assert_eq!(sll_instr.rt, get_register_number(rt).unwrap());
        assert_eq!(sll_instr.rd, get_register_number(rd).unwrap());
        assert_eq!(sll_instr.shamt.to_string(), shamt);
    }

    #[test]
    fn test_create_ori_instruction() {
        // ORI rt, rs, imm
        let rt = "$t0";
        let rs = "$t1";
        let imm = "2"; //0010
        let instr = "ori";
        let ori_instr = ITypeInstruction::new(instr, rs, rt, imm, 0).unwrap();
        assert_eq!(ori_instr.op, get_numeric_op(instr).unwrap());
        assert_eq!(ori_instr.get_rs(), get_register_number("$t1").unwrap());
        assert_eq!(ori_instr.get_rt(), get_register_number("$t0").unwrap());
        assert_eq!(ori_instr.get_imm(), 2);
    }

    #[test]
    fn test_create_srl_instruction() {
        // SRL rd, rt, sa
        let rd = "$t0";
        let rt = "$t1";
        let shamt = "4";
        let instr = "srl";
        let srl_instr = RTypeInstruction::shift(instr, rd, rt, shamt).unwrap();
        assert_eq!(srl_instr.get_op(), get_numeric_op(instr).unwrap());
        assert_eq!(srl_instr.get_rd(), get_register_number(rd).unwrap());
        assert_eq!(srl_instr.get_rt(), get_register_number(rt).unwrap());
        assert_eq!(srl_instr.get_shamt(), u5::new(4));
        assert_eq!(srl_instr.get_func(), get_func(instr).unwrap());
    }

    #[test]
    fn test_create_sra_instruction() {
        // SRL rd, rt, sa
        let rd = "$t0";
        let rt = "$t1";
        let shamt = "4";
        let instr = "sra";
        let srl_instr = RTypeInstruction::shift(instr, rd, rt, shamt).unwrap();
        assert_eq!(srl_instr.get_op(), get_numeric_op(instr).unwrap());
        assert_eq!(srl_instr.get_rd(), get_register_number(rd).unwrap());
        assert_eq!(srl_instr.get_rt(), get_register_number(rt).unwrap());
        assert_eq!(srl_instr.get_shamt(), u5::new(4));
        assert_eq!(srl_instr.get_func(), get_func(instr).unwrap());
    }

    #[test]
    fn test_extract_instruction_successful() {
        let instruction = "label1: addi  $t1, $zero, 1   # A comment";
        let expected = " addi  $t1, $zero, 1   ";
        let got = remove_comment_and_label(instruction);
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn test_extract_instruction_only_label() {
        let instruction = "label1:";
        let got = remove_comment_and_label(instruction);
        assert!(got.is_none());
    }

    #[test]
    fn test_extract_instruction_label_and_comment() {
        let instruction = "label1: # A comment";
        let got = remove_comment_and_label(instruction);
        assert!(got.is_none());
    }

    #[test]
    fn test_extract_instruction_only_comment() {
        let instruction = " #this is a comment";
        let got = remove_comment_and_label(instruction);
        assert!(got.is_none());
    }

    #[test]
    fn test_parse_add_instruction_with_comment() {
        let line = "add $t0, $t1, $t2 # hello";
        let maybe_instr = parse_instruction(line.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        let instr = maybe_instr.unwrap().unwrap();
        match instr {
            Instruction::RType(instruction) => {
                assert_eq!(instruction.get_op(), get_numeric_op("add").unwrap());
                assert_eq!(instruction.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instruction.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instruction.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instruction.get_func(), get_func("add").unwrap());
                assert_eq!(instruction.get_shamt(), u5::new(0));
            }
            _ => panic!("Expected R-Type Instruction"),
        };
    }

    #[test]
    fn test_remove_comment_label_only_label() {
        let line = "label:";
        let maybe_label = remove_comment_and_label(line);
        assert!(maybe_label.is_none());
    }

    #[test]
    fn test_remove_comment_label_only_comment() {
        let line = "# this is a comment";
        let maybe_comment = remove_comment_and_label(line);
        assert!(maybe_comment.is_none());
    }

    #[test]
    fn test_remove_comment_label_whole_line() {
        let whole_line = "label: add $t0, $t1, $t2 # this is a comment";
        let maybe_instr = remove_comment_and_label(whole_line);
        assert!(maybe_instr.is_some());
        assert_eq!(maybe_instr.unwrap().as_str(), " add $t0, $t1, $t2 "); // Whitespace is preserved
    }

    #[test]
    fn test_has_instruction_after_removing_label_comment() {
        let line_after_removed_label_comment = " add $t0, $t1, $t2 "; // This is what string should look like after removing label and comment
        assert!(has_instruction(line_after_removed_label_comment));
    }

    #[test]
    fn test_contains_supported_instruction_after_removing_label_comment() {
        let line_after_removed_label_comment = " add $t0, $t1, $t2 ";
        assert!(contains_supported_instruction(
            line_after_removed_label_comment
        ));
    }

    #[test]
    fn test_parse_without_comment_label() {
        let comment_label_removed = " add $t0, $t1, $t2 ";
        let maybe_instr =
            parse_instruction_without_comment_label(comment_label_removed, &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
    }

    #[test]
    fn test_parse_beq_no_comment_no_label() {
        let only_beq_instr = "beq $t0, $t1, 8"; // format: beq rs, rt, imm
        let maybe_instr =
            parse_instruction_without_comment_label(only_beq_instr, &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        let instr = maybe_instr.unwrap();
        match instr {
            Instruction::IType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("beq").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_imm(), 4);
            }
            _ => {
                panic!("");
            }
        }
    }

    #[test]
    fn test_parse_add_instruction_with_label() {
        let line = "label: add $t0, $t1, $t2";
        let maybe_instr = parse_instruction(line.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        match maybe_instr.unwrap().unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("add").unwrap());
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.get_func(), get_func("add").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
            }
            _ => panic!("Expected R-Type instruction. Found other type."),
        };
    }

    #[test]
    fn test_parse_add_instruction_with_label_and_comment() {
        let line = "label: add $t0, $t1, $t2 # comment";
        let maybe_instr = parse_instruction(line.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        match maybe_instr.unwrap().unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("add").unwrap());
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.get_func(), get_func("add").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
            }
            _ => panic!("Expected R-Type instruction. Found other type."),
        };
    }

    #[test]
    fn test_parse_instruction_missing_params() {
        let invalid_instr = "add $t1, $t2";
        let maybe_instr = parse_instruction(invalid_instr.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_err());
    }

    #[test]
    fn test_parse_instruction_invalid_register() {
        let invalid_instr = "add $t1, $t2, $a9";
        let maybe_instr = parse_instruction(invalid_instr.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_err());
    }

    #[test]
    fn test_parse_instruction_invalid_instr() {
        let invalid_instr = "abd $t1, $t2, $t3";
        let maybe_instr = parse_instruction(invalid_instr.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_err());
        if let Err(e) = maybe_instr {
            assert_eq!(
                "Could not parse instruction from given line abd $t1, $t2, $t3",
                e
            );
        }
    }

    #[test]
    fn test_parse_add_immediate() {
        let add_imm = "addi $t0, $t1, 5";
        let maybe_instr = parse_instruction(add_imm.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        match maybe_instr.unwrap().unwrap() {
            Instruction::IType(instr) => {
                assert_eq!(instr.op, get_numeric_op("addi").unwrap());
                assert_eq!(instr.rt, get_register_number("$t0").unwrap());
                assert_eq!(instr.rs, get_register_number("$t1").unwrap());
                assert_eq!(instr.imm, 5);
            }
            _ => panic!("Expected I-Type instruction. Got other type."),
        };
    }

    #[test]
    fn test_parse_sra() {
        let instr = "sra $t0, $t1, 4";
        let maybe_instr = parse_instruction(instr.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());

        match maybe_instr.unwrap().unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.op, get_numeric_op("sra").unwrap());
                assert_eq!(instr.rd, get_register_number("$t0").unwrap());
                assert_eq!(instr.rt, get_register_number("$t1").unwrap());
                assert_eq!(instr.shamt, u5::new(4));
                assert_eq!(instr.func, get_func("sra").unwrap());
            }
            _ => panic!("Expected R-Type instruction. Got other type."),
        };
    }

    #[test]
    fn test_parse_srl() {
        let instr = "srl $t1, $t2, 7";
        let maybe_instr = parse_instruction(instr.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());

        match maybe_instr.unwrap().unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.op, get_numeric_op("srl").unwrap());
                assert_eq!(instr.rd, get_register_number("$t1").unwrap());
                assert_eq!(instr.rt, get_register_number("$t2").unwrap());
                assert_eq!(instr.shamt, u5::new(7));
                assert_eq!(instr.func, get_func("srl").unwrap());
            }
            _ => panic!("Expected R-Type instruction. Got other type."),
        };
    }

    #[test]
    fn test_parse_only_label() {
        let label = "label:";
        let maybe_instr = parse_instruction(label.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        assert!(maybe_instr.unwrap().is_none());
    }

    #[test]
    fn test_parse_with_tab_indentation() {
        let line = "    add $t0, $t1, $t2";
        let maybe_instr = parse_instruction(line.to_string(), &HashMap::new(), 0);
        assert!(maybe_instr.is_ok());
        let ok_instr = maybe_instr.unwrap();
        assert!(ok_instr.is_some());
        let instr = ok_instr.unwrap();
        match instr {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("add").unwrap());
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
                assert_eq!(instr.get_func(), get_func("add").unwrap());
            }
            _ => panic!("Expected R-Type instruction. Got other type."),
        };
    }

    #[test]
    fn test_parse_memory_access_arguments() {
        let argument_string = "$t2, 4($t5)";
        let got = parse_memory_access_arguments(argument_string).unwrap();
        println!("{:?}", parse_memory_access_arguments(argument_string));
        assert_eq!(
            get_register_number("$t2"),
            get_register_number(&got[0].to_string())
        );
        assert_eq!(
            get_register_number("$t5"),
            get_register_number(&got[2].to_string())
        );
        assert_eq!("4", &got[1].to_string());
    }

    #[test]
    fn test_parse_memory_access_arguments_faulty() {
        let one_argument = "$t2";
        let two_arguments: &str = "t2, $t3";
        let brute_force: &str = "(), base(offset)()()()()()";

        assert!(parse_memory_access_arguments(one_argument).is_err());
        assert!(parse_memory_access_arguments(two_arguments).is_err());
        assert!(parse_memory_access_arguments(brute_force).is_ok());
    }

    #[test]
    fn test_parse_jr_successful() {
        let line = String::from("label: jr $t0 #comment");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let res = parse_res.unwrap();
        assert_eq!(res.unwrap().to_hex_string(), "0x01000008");
    }

    #[test]
    fn test_parse_lw_successful() {
        let line = String::from("label: lw $t0, 5($t1)");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let res = parse_res.unwrap();
        assert_eq!(res.unwrap().to_hex_string(), "0x8d280005");
    }

    #[test]
    fn test_parse_sw_successful() {
        let line = String::from("label: sw $t0, 5($t1)");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let res = parse_res.unwrap();
        assert_eq!(res.unwrap().to_hex_string(), "0xad280005");
    }

    #[test]
    fn test_parse_jr_unsuccessful() {
        let line = String::from("label: jr #comment");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_err());
    }

    #[test]
    fn test_parse_lw_unsuccessful() {
        let line = String::from("label: lw $t0, ($t1) #comments are nice");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_err());
    }

    #[test]
    fn test_parse_sw_unsuccessful() {
        let line = String::from("label: sw $t0 ($t1) #comments are not nice");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_err());
    }

    #[test]
    fn test_parse_sll_successful() {
        let line = String::from("sll $t0, $t1, 1"); // t0 = t1 << 1
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), u6::new(0));
                assert_eq!(instr.get_func(), u6::new(0));
                assert_eq!(instr.get_rs(), u5::new(0));
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(1));
                assert_eq!(instr.to_hex_string(), "0x00094040");
            }
            _ => panic!("Expected R-Type Instruction"),
        }
    }

    #[test]
    fn test_parse_slt_successful() {
        let line = String::from("slt $t0, $t1, $t2");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("slt").unwrap());
                assert_eq!(instr.get_func(), get_func("slt").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.to_hex_string(), "0x012a402a");
            }
            _ => panic!("Expected R-Type Instruction"),
        }
    }

    #[test]
    fn test_parse_and_successful() {
        let line = String::from("and $t0, $t1, $t2");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("and").unwrap());
                assert_eq!(instr.get_func(), get_func("and").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.to_hex_string(), "0x012a4024");
            }
            _ => panic!("Expected R-Type Instruction"),
        }
    }

    #[test]
    fn test_parse_or_successful() {
        let line = String::from("or $t0, $t1, $t2");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("or").unwrap());
                assert_eq!(instr.get_func(), get_func("or").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.to_hex_string(), "0x012a4025");
            }
            _ => panic!("Expected R-Type Instruction"),
        }
    }

    #[test]
    fn test_parse_nor_successful() {
        let line = String::from("nor $t0, $t1, $t2");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::RType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("nor").unwrap());
                assert_eq!(instr.get_func(), get_func("nor").unwrap());
                assert_eq!(instr.get_shamt(), u5::new(0));
                assert_eq!(instr.get_rd(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t2").unwrap());
                assert_eq!(instr.to_hex_string(), "0x012a4027");
            }
            _ => panic!("Expected R-Type Instruction"),
        }
    }

    #[test]
    fn test_addi_canvas_example() {
        // addi rt, rs, imm
        let line = String::from("addi $t0, $zero, 64");
        let parse_res = parse_instruction(line, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::IType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("addi").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$zero").unwrap());
                assert_eq!(instr.get_imm(), 64);
                assert_eq!(instr.to_hex_string(), "0x20080040");
            }
            _ => panic!("Expected R-Type Instruction"),
        }
    }

    #[test]
    fn test_addi_negatives(){
        let line1 = String::from("addi $t0, $t1, -10");
        let line2 = String::from("addi $t0, $t1, -20");
        let line3 = String::from("addi $t0, $t1, -100");


        // imm = -10
        let parse_res = parse_instruction(line1, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::IType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("addi").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_imm(), -10);
                assert_eq!(instr.to_hex_string(), "0x2128fff6");
                
            }
            _ => panic!("weird.. thought I had myself an ITypeInstruction..."),
        }

        // imm = -20
        let parse_res = parse_instruction(line2, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::IType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("addi").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_imm(), -20);
                assert_eq!(instr.to_hex_string(), "0x2128ffec");
                
            }
            _ => panic!("weird.. thought I had myself an ITypeInstruction..."),
        }


        // imm = -100
        let parse_res = parse_instruction(line3, &HashMap::new(), 0);
        assert!(parse_res.is_ok());
        let maybe_instr = parse_res.unwrap();
        assert!(maybe_instr.is_some());
        match maybe_instr.unwrap() {
            Instruction::IType(instr) => {
                assert_eq!(instr.get_op(), get_numeric_op("addi").unwrap());
                assert_eq!(instr.get_rt(), get_register_number("$t0").unwrap());
                assert_eq!(instr.get_rs(), get_register_number("$t1").unwrap());
                assert_eq!(instr.get_imm(), -100);
                assert_eq!(instr.to_hex_string(), "0x2128ff9c");
                
            }
            _ => panic!("weird.. thought I had myself an ITypeInstruction..."),
        }
    }

    #[test]
    fn test_terminate_program_instr(){
        let exit = Instruction::TerminateInstructionType(TerminateInstruction{});
        assert_eq!(exit.to_hex_string(), "0xFFFFFFFF");
        assert_eq!(exit.to_bin_string(), "1".repeat(32).to_string());
    }

    #[test]
    fn test_to_mnemonic_str() {
        assert_eq!("nop".to_string(), NopTypeInstruction{}.to_mnemonic_string());
        assert_eq!("exit".to_string(), TerminateInstruction{}.to_mnemonic_string());
        assert_eq!("jr $t1", JRTypeInstruction::new("jr", "$t1").unwrap().to_mnemonic_string());
        assert_eq!("lw $t1, 1($t2)", MemoryAccessTypeInstruction::new("lw", "$t1", "1", "$t2").unwrap().to_mnemonic_string());
        assert_eq!("sw $t1, 1($t2)", MemoryAccessTypeInstruction::new("sw", "$t1", "1", "$t2").unwrap().to_mnemonic_string());
        assert_eq!("add $t0, $t1, $t2", RTypeInstruction::new("add", "$t1", "$t2", "$t0").unwrap().to_mnemonic_string());
        assert_eq!("addi $t0, $t1, 1", ITypeInstruction::new("addi", "$t1", "$t0", "1", 0).unwrap().to_mnemonic_string());
        
    }

    #[test]
    fn test_get_operation() {
        // test everything but nop and exit!
        assert_eq!(get_operation(u6::new(0), u6::new(32)).unwrap(),"add");
        assert_eq!(get_operation(u6::new(0), u6::new(34)).unwrap(), "sub");
        assert_eq!(get_operation(u6::new(0), u6::new(36)).unwrap(), "and");
        assert_eq!(get_operation(u6::new(0), u6::new(37)).unwrap(), "or");
        assert_eq!(get_operation(u6::new(0), u6::new(39)).unwrap(), "nor");
        assert_eq!(get_operation(u6::new(0), u6::new(42)).unwrap(), "slt");
        assert_eq!(get_operation(u6::new(35), u6::new(0)).unwrap(), "lw");
        assert_eq!(get_operation(u6::new(43), u6::new(0)).unwrap(), "sw");
        assert_eq!(get_operation(u6::new(4), u6::new(0)).unwrap(), "beq");
        assert_eq!(get_operation(u6::new(8), u6::new(0)).unwrap(), "addi");
        assert_eq!(get_operation(u6::new(0), u6::new(0)).unwrap(), "sll");
        assert_eq!(get_operation(u6::new(2), u6::new(0)).unwrap(), "j");
        assert_eq!(get_operation(u6::new(0),u6::new(8)).unwrap(), "jr");
        assert_eq!(get_operation(u6::new(13), u6::new(0)).unwrap(), "ori");
        assert_eq!(get_operation(u6::new(0), u6::new(2)).unwrap(), "srl");
        assert_eq!(get_operation(u6::new(0), u6::new(3)).unwrap(), "sra");
    }

}
