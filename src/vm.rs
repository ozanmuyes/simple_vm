use crate::opcodes;

pub struct VM {
    text_section: Vec<u8>,
    data_section: Vec<u8>,

    // FIXME State is the duplicate of here
    // special registers
    /// Instruction pointer
    ip: usize,
    /// Stack pointer
    sp: usize,
    //

    // general purpose registers
    r: [u8; 8],
    // TODO stack, etc.
}

impl std::fmt::Debug for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VM")
            // .field("text_section", &self.text_section)
            // .field("data_section", &self.data_section)
            .field("ip", &self.ip)
            .field("sp", &self.sp)
            .field("r", &self.r)
            .finish()
    }
}

pub enum TickResult {
    CanContinue,
    Halted,
    Error(i32),
}

const SECTION_HEADER_SIZE: usize = 1; // bytes

// impl Drop for VM {
//     fn drop(&mut self) {
//         todo!("teardown the isolate");

//         // TODO close opened files, etc.
//     }
// }

impl VM {
    pub fn new(bytecode: Vec<u8>) -> VM {
        let section_header = &bytecode[0..SECTION_HEADER_SIZE];
        let code_ends_at: usize = SECTION_HEADER_SIZE + section_header[0] as usize;

        VM {
            ip: 0,
            sp: 0,
            r: [0; 8],
            text_section: bytecode[SECTION_HEADER_SIZE..code_ends_at].to_vec(),
            data_section: bytecode[code_ends_at..].to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn run(&mut self) -> Result<(), i32> {
        loop {
            println!("{:?}", &self);

            match self.tick() {
                TickResult::CanContinue => continue,
                TickResult::Halted => return Ok(()),
                TickResult::Error(error_code) => return Err(error_code),
            }
        }
    }

    // // NOTE Because of this each opcode should be 24-bit
    // fn fetch(&self) -> (u8, u8, u8) {
    //     // TODo
    // }

    // fn decode_and_execute() -> TickResult {
    //     // TODO
    // }

    pub fn tick(&mut self) -> TickResult {
        let opcode = self.text_section[self.ip];

        // FIXME for match to enforce all available options have `opcodes` as (u8) enum
        let result = match opcode {
            opcodes::NOOP => TickResult::CanContinue,
            opcodes::HALT => TickResult::Halted,
            // #region MOVs
            opcodes::MOV0 => {
                self.ip += 1;
                self.r[0] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV1 => {
                self.ip += 1;
                self.r[1] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV2 => {
                self.ip += 1;
                self.r[2] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV3 => {
                self.ip += 1;
                self.r[3] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV4 => {
                self.ip += 1;
                self.r[4] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV5 => {
                self.ip += 1;
                self.r[5] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV6 => {
                self.ip += 1;
                self.r[6] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            opcodes::MOV7 => {
                self.ip += 1;
                self.r[7] = self.text_section[self.ip];
                TickResult::CanContinue
            }
            // #endregion MOVs
            opcodes::SYSCALL => {
                match self.r[2] {
                    0x4 => {
                        // write
                        if self.r[3] == 1 {
                            // stdout

                            let start_pos: usize = self.r[4].into();
                            let null_pos = self
                                .data_section
                                .iter()
                                .skip(start_pos)
                                .position(|&c| c == 0);
                            let null_pos = match null_pos {
                                Some(pos) => pos,
                                None => return TickResult::Error(-3), // FIXME use MEMORY_OOB_NZ (memory ou-of-bound while seeking null-terminate)
                            };

                            let buf = std::str::from_utf8(
                                &self.data_section[start_pos..start_pos + null_pos],
                            )
                            .unwrap();

                            print!("{}", buf);
                        }
                    }

                    _ => return TickResult::Error(-2), // FIXME use enum to indicate UNKNOWN_SYSCALL (instead of hardcoded `-2`)
                }

                TickResult::CanContinue
            }
            //
            _ => return TickResult::Error(1), // FIXME Instead 1 write UNKNOWN_OPCODE
        };

        self.ip += 1;

        result
    }
}
