//!  Multiplexor unit for the processor.
//!  
//! Author: mai21asm, c19hln
//! Since: 2022-11-24
//! Version: 1.0
pub struct Multiplexor {
    signal: bool,
}

impl Multiplexor {

    pub fn new() -> Multiplexor {
        Multiplexor { signal: false }
    }

    pub fn set_signal(&mut self, signal: bool) {
        self.signal = signal;
    }

    #[allow(unused)] // might be needed
    pub fn get_signal(&self) -> bool {
        self.signal
    }

    pub fn multiplex(&self, val_1: Option<i32>, val_2: i32) -> i32 {
        if self.signal {
            return val_1.unwrap();
        }

        val_2
    }
}
