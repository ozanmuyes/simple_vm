mod opcodes;
mod vm;

use vm::{TickResult, VM};

fn main() -> Result<(), i32> {
    let bytecode = &[
        // #region section_header
        0x2, // .text section length (in bytes)
        //
        // #endregion section_header

        // #region .text section
        opcodes::NOOP,
        opcodes::HALT,
        //
        // #endregion .text section

        // #region .data
        42,
        // #endregion .data
    ];

    // host application should exit by 0 code
    // let bytecode = &[OP_HALT, 1]; // host application should exit by 1 code
    // let bytecode = &[OP_HALT, 42]; // host application should exit by 1 code
    let mut isolate: VM<'static> = VM::new(bytecode);

    // match isolate.run() {
    //     Ok(_) => return Ok(()),
    //     Err(error) => return Err(error),
    // }

    // OR

    loop {
        // TODO console.log(vm.get_state())
        match isolate.tick() {
            TickResult::CanContinue => continue,
            TickResult::Halted => return Ok(()),
            TickResult::Error(error) => return Err(error),
        }
    }

    // OR JUST (notice return result)
    // isolate.run()
}
