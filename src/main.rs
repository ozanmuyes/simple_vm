mod opcodes;
mod vm;

use vm::VM;

fn build_bytecode(text_section: &[u8], data_section: &[u8]) -> Vec<u8> {
    // vec![text_section.len() as u8].append(text_section)
    let mut arr = vec![text_section.len() as u8];
    arr.extend_from_slice(text_section);
    arr.extend_from_slice(data_section);
    arr
}

fn main() -> Result<(), i32> {
    let mut data_section = vec![];
    data_section.extend_from_slice("yo\0".as_bytes());
    data_section.extend_from_slice("Hello World!\n\0".as_bytes());

    #[rustfmt::skip]
    let bytecode = build_bytecode(
        &[
            opcodes::MOV2, 0x4, // which syscall (0x4 in this case -> write)
            opcodes::MOV3, 0x1, // syscall arg 1 (0x1 in this case -> stdout)
            opcodes::MOV4, 0x3, // syscall arg 2 (data section address of buffer)
            opcodes::SYSCALL,
            opcodes::HALT,
        ],
        &data_section[..],
    );

    let mut isolate: VM = VM::new(bytecode);

    // loop {
    //     println!("{:?}", isolate);

    //     match isolate.tick() {
    //         TickResult::CanContinue => continue,
    //         TickResult::Halted => return Ok(()),
    //         TickResult::Error(error) => return Err(error),
    //     }
    // }

    // OR

    match isolate.run() {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error),
    }

    // OR JUST (notice return result)
    // isolate.run()
}
