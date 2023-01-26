//!  This is the alu control unit of the MIPS processor.
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-25
//! Version: 1.0
use ux::{u4, u6};

pub fn get_alu_signal(alu_op_0: bool, alu_op_1: bool, funct_field: u6) -> u4 {
    if alu_op_1 && alu_op_0 {
        return u4::new(1);
    }
    if alu_op_1 && !alu_op_0 {
        // R-type instruction
        get_rtype_signal(funct_field)
    } else {
        if alu_op_0 {
            return u4::new(6);
        }
        u4::new(2)
    }
}

pub fn get_rtype_signal(funct_field: u6) -> u4 {
    let funct_field_val: u8 = funct_field.into();
    match funct_field_val {
        32 | 8 => u4::new(2),
        34 => u4::new(6),
        36 => u4::new(0),
        37 => u4::new(1),
        42 => u4::new(7),
        3 => u4::new(13),  
        2 => u4::new(14),  
        0 => u4::new(0),   
        39 => u4::new(12), 
        _ => panic!(
            "Invalid funct_field for R-Type ALU-signal! Got value: {}",
            funct_field
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtype_signals() {
        assert_eq!(u4::new(2), get_rtype_signal(u6::new(32)));
        assert_eq!(u4::new(6), get_rtype_signal(u6::new(34)));
        assert_eq!(u4::new(0), get_rtype_signal(u6::new(36)));
        assert_eq!(u4::new(1), get_rtype_signal(u6::new(37)));
        assert_eq!(u4::new(7), get_rtype_signal(u6::new(42)));
    }

    #[test]
    fn test_alu_signal() {
        let lw_signal = get_alu_signal(false, false, u6::max_value());
        let sw_signal = get_alu_signal(false, false, u6::min_value());
        assert_eq!(lw_signal, sw_signal);
        assert_eq!(lw_signal, u4::new(2));

        let beq_signal_1 = get_alu_signal(true, false, u6::max_value());
        let beq_signal_2 = get_alu_signal(true, false, u6::min_value());
        assert_eq!(beq_signal_1, beq_signal_2);
        assert_eq!(beq_signal_1, u4::new(6));

        let add_signal = get_alu_signal(false, true, u6::new(32));
        assert_eq!(add_signal, u4::new(2));

        let sub_signal = get_alu_signal(false, true, u6::new(34));
        assert_eq!(sub_signal, u4::new(6));
        
        let and_signal = get_alu_signal(false, true, u6::new(36));
        assert_eq!(and_signal, u4::new(0));

        let or_signal = get_alu_signal(false, true, u6::new(37));
        assert_eq!(or_signal, u4::new(1));

        let slt_signal = get_alu_signal(false, true, u6::new(42));
        assert_eq!(slt_signal, u4::new(7));

        let sra_sig = get_alu_signal(false, true, u6::new(3));
        assert_eq!(sra_sig, u4::new(13));

        let srl_sig = get_alu_signal(false, true, u6::new(2));
        assert_eq!(srl_sig, u4::new(14));
    }
}
