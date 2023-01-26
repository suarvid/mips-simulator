/// shift left two component in MIPS if needed..

pub struct ShiftLeftTwo{}

impl ShiftLeftTwo {
    pub fn new() -> ShiftLeftTwo {
        ShiftLeftTwo{}
    }

    pub fn shift(&self, target: u32) -> u32{
        target << 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift()  {
        let shift_left = ShiftLeftTwo{}; 
        assert_eq!(shift_left.shift(8), 32);
    }
}