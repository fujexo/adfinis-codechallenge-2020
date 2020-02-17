use std::io;
use std::process;
extern crate csv;
mod advm;
use advm::virtual_machine;

pub fn run() {
    let vm = virtual_machine::Vm::new();
    let program = read_program();
    let debug = false;

    virtual_machine::execute_program(vm, program, debug);
}

fn read_program() -> Vec<virtual_machine::Instruction> {
    let mut program: Vec<virtual_machine::Instruction> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Examine our Result.
        // If there was no problem, print the record.
        // Otherwise, print the error message and quit the program.
        match result {
            Ok(record) => {
                //println!("{:?}", instruction);
                program.push(record);
            }
            Err(err) => {
                eprintln!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }

    program
}
