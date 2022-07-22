// // NOTE Let NOOP stay intact
// pub const NOOP: u8 = 0x0;
// pub const MOV0: u8 = 0x10;
// pub const MOV1: u8 = 0x11;
// pub const MOV2: u8 = 0x12;
// pub const MOV3: u8 = 0x13;
// pub const MOV4: u8 = 0x14;
// pub const MOV5: u8 = 0x15;
// pub const MOV6: u8 = 0x16;
// pub const MOV7: u8 = 0x17;
// //
// pub const SYSCALL: u8 = 0x2;
// //

// pub const YIELD: u8 = 0x7C;
// pub const RET: u8 = 0x7D;
// pub const HALT: u8 = 0x7E;
// // NOTE Max 0x7F

pub enum Opcode {
    NOOP,
    MOV(u8), // target register index (0x10 -> 0th index, 0x11 -> 1st index, ..., ... -> 7.th index)
    LOAD(u8),
    STORE(u8),
    SYSCALL(u8), // syscall number (0x20 -> open, 0x21 -> read, ..., ... -> recv)
    YIELD,
    RET,
    HALT,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x0 => return Opcode::NOOP,
            //
            0x10 => return Opcode::MOV(0),
            0x11 => return Opcode::MOV(1),
            0x12 => return Opcode::MOV(2),
            0x13 => return Opcode::MOV(3),
            0x14 => return Opcode::MOV(4),
            0x15 => return Opcode::MOV(5),
            0x16 => return Opcode::MOV(6),
            0x17 => return Opcode::MOV(7),
            // 0x18 - 0x1F
            0x20 => return Opcode::LOAD(0),
            0x21 => return Opcode::LOAD(1),
            0x22 => return Opcode::LOAD(2),
            0x23 => return Opcode::LOAD(3),
            0x24 => return Opcode::LOAD(4),
            0x25 => return Opcode::LOAD(5),
            0x26 => return Opcode::LOAD(6),
            0x27 => return Opcode::LOAD(7),
            // 0x28 - 0x2F
            0x30 => return Opcode::STORE(0),
            0x31 => return Opcode::STORE(1),
            0x32 => return Opcode::STORE(2),
            0x33 => return Opcode::STORE(3),
            0x34 => return Opcode::STORE(4),
            0x35 => return Opcode::STORE(5),
            0x36 => return Opcode::STORE(6),
            0x37 => return Opcode::STORE(7),
            // 0x38 - 0xD4
            // 0xD5 - 0xFF
        }
    }
}
