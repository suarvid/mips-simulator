
/// Adder for the MIPS simulator in case we need it..

/* Just an empty struct for now */
pub struct Adder {

}

impl Adder {

    pub fn new() -> Adder {
        Adder{}
    }


    pub fn add(&self, x:i32, y:i32) -> i32 {
        //TODO - do we need to simulate a real 32-bit full adder or is this ok??
        x+y
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add()  {
        let mut adder = Adder::new();
        assert_eq!(adder.add(4,0), 4);
    }
}
