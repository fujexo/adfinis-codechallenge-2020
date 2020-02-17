pub mod virtual_machine {

    use serde::Deserialize;
    use std::process;

    #[derive(Debug)]
    pub struct Vm {
        memory: [f64; 7],
        pub counter: usize,
        register: f64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Instruction {
        counter: u32,
        instruction: String,
        #[serde(deserialize_with = "csv::invalid_option")]
        value1: Option<f64>,
        #[serde(deserialize_with = "csv::invalid_option")]
        value2: Option<f64>,
    }

    impl Vm {
        pub fn new() -> Vm {
            let memory = [0.0; 7];
            let counter = 0;
            let register = 0.0;

            Vm {
                memory,
                counter,
                register,
            }
        }
        pub fn dump(vm: &Vm) {
            println!("-------------------------");
            println!("VM: {:?}", vm);
        }
    }

    pub fn execute_program(mut vm: Vm, program: Vec<Instruction>, debug: bool) {
        loop {
            let counter = vm.counter.clone();

            match program[counter].instruction.as_ref() {
                "val" => {
                    vm = instruction::val(
                        vm,
                        program[counter].value1.unwrap(),
                        program[counter].value2.unwrap(),
                    )
                }
                "load" => vm = instruction::load(vm, program[counter].value1.unwrap()),
                "div" => vm = instruction::div(vm, program[counter].value1.unwrap()),
                "stor" => vm = instruction::stor(vm, program[counter].value1.unwrap()),
                "gt" => {
                    vm = instruction::gt(
                        vm,
                        program[counter].value1.unwrap(),
                        program[counter].value2.unwrap(),
                    )
                }
                "inc" => vm = instruction::inc(vm),
                "mul" => vm = instruction::mul(vm, program[counter].value1.unwrap()),
                "pow" => vm = instruction::pow(vm, program[counter].value1.unwrap()),
                "add" => vm = instruction::add(vm, program[counter].value1.unwrap()),
                "prt" => instruction::prt(&vm),
                "jmp" => vm = instruction::jmp(vm, program[counter].value1.unwrap()),

                _ => panic!("Instruction not found"),
            }
            if debug {
                Vm::dump(&vm);
            }

            if counter >= program.len() - 1 {
                if debug {
                    println!("No more instructions");
                }
                process::exit(0);
            }
        }
    }

    pub mod instruction {
        pub fn val(mut vm: super::Vm, value: f64, memory_address: f64) -> super::Vm {
            /*
             * This method sets memory a address to value v
             * 00, val, <value>, <memory>
             * */
            vm.memory[memory_address as usize] = value;
            vm.counter += 1;
            vm
        }
        pub fn load(mut vm: super::Vm, memory_address: f64) -> super::Vm {
            /*
             * This method loads memory address a into register
             * 00, load, <memory>
             * */
            vm.register = vm.memory[memory_address as usize];
            vm.counter += 1;
            vm
        }
        pub fn stor(mut vm: super::Vm, memory_address: f64) -> super::Vm {
            /*
             * This method stores the register into memory address a
             * 00, stor, <memory>
             * */
            vm.memory[memory_address as usize] = vm.register;
            vm.counter += 1;
            vm
        }
        pub fn add(mut vm: super::Vm, memory_address: f64) -> super::Vm {
            /*
             * This method adds the value in memory address a to the register
             * 00, add, <memory>
             * */
            vm.register += vm.memory[memory_address as usize];
            vm.counter += 1;
            vm
        }
        pub fn div(mut vm: super::Vm, memory_address: f64) -> super::Vm {
            /*
             * This method divides the register by value in memory address a
             * 00, div, <memory>
             * */
            vm.register /= vm.memory[memory_address as usize];
            vm.counter += 1;
            vm
        }
        pub fn mul(mut vm: super::Vm, memory_address: f64) -> super::Vm {
            /*
             * This method multiplies the register by value in memory address a
             * 00, mul, <memory>
             * */
            vm.register *= vm.memory[memory_address as usize];
            vm.counter += 1;
            vm
        }
        pub fn pow(mut vm: super::Vm, memory_address: f64) -> super::Vm {
            /*
             * This method raises the register to the power of memory address a
             * 00, pow, <memory>
             * */
            vm.register = vm.register.powf(vm.memory[memory_address as usize]);
            vm.counter += 1;
            vm
        }
        pub fn inc(mut vm: super::Vm) -> super::Vm {
            /*
             * This method increments the register by one
             * 00, inc
             * */
            vm.register += 1.0;
            vm.counter += 1;
            vm
        }
        pub fn prt(vm: &super::Vm) {
            /*
             * This method prints the register
             * 00, prt
             * */
            println!("{}", vm.register);
        }
        pub fn gt(mut vm: super::Vm, memory_address: f64, instruction: f64) -> super::Vm {
            /*
             * This method jumps to instriction i if register ist greater than memory address a
             * 00, gt, <memory> <instruction>
             * */
            if vm.register > vm.memory[memory_address as usize] {
                vm.counter = instruction as usize;
            } else {
                vm.counter += 1;
            }
            vm
        }
        pub fn jmp(mut vm: super::Vm, instruction: f64) -> super::Vm {
            /*
             * This method jumps to instruction i
             * 00, jmp, <instruction>
             * */
            vm.counter = instruction as usize;
            vm
        }
    }
}
