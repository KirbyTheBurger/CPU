pub struct CPU {
    reg: [u16; 8],
    mem: [u16; 0xFFFF],
    pc: u16,
    sp: u16,
    flags: u8,
    running: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: [0; 8],
            mem: [0; 0xFFFF],
            pc: 0,
            sp: 0,
            flags: 0,
            running: true,
        }
    }
}