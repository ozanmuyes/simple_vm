use crate::opcodes;

pub struct VM<'vm> {
    text_section: &'vm [i8],
    data_section: &'vm [i8],

    // FIXME State is the duplicate of here
    // special registers
    /// Instruction pointer
    ip: usize,
    /// Stack pointer
    sp: usize,
    //

    // TODO general purpose registers (r0, r1, ...)

    // TODO stack, etc.
}

pub enum TickResult {
    CanContinue,
    Halted,
    Error(i32),
}

const SECTION_HEADER_SIZE: usize = 1; // bytes

pub struct State {
    // special registers
    /// Instruction pointer
    ip: usize,
    /// Stack pointer
    sp: usize,
    //

    // TODO general purpose registers (r0, r1, ...)

    // TODO stack, etc.
}

impl<'a> VM<'a> {
    pub fn new(bytecode: &'a [i8]) -> VM {
        // sections
        let section_header = &bytecode[0..SECTION_HEADER_SIZE];
        let lala: i32 = (*section_header.get(0).unwrap()).into();
        let code_ends_at: usize = SECTION_HEADER_SIZE + lala as usize;

        VM {
            ip: 0,
            sp: 0,
            // TODO state
            text_section: &bytecode[SECTION_HEADER_SIZE..code_ends_at],
            data_section: &bytecode[code_ends_at..],
            // TODO
        }
    }

    pub fn run(&mut self) -> (Result<(), i32>, State) {
        loop {
            match self.tick() {
                TickResult::CanContinue => continue,
                TickResult::Halted => {
                    todo!("teardown the vm");

                    return Ok(());
                }
                TickResult::Error(error_code) => {
                    todo!("teardown the vm");

                    return Err(error_code);
                }
            }
        }
    }

    // if return false, then halted
    pub fn tick(&mut self) -> TickResult {
        let opcode = self.text_section[self.ip];

        let result = match opcode {
            opcodes::NOOP => TickResult::CanContinue,
            opcodes::HALT => TickResult::Halted,
            //
            _ => return TickResult::Error(1), // FIXME Instead 1 write UNKNOWN_OPCODE
        };

        self.ip += 1;

        result
    }
}
