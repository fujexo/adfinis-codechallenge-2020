# Weclome to the advm challenge

What does the following advm programm do: https://git.io/JvCV6?

Create the advm interpreter and run the above program.


## interpreter requirements

The advm virtual machine has:

* One register of type float
* 7 memory addresses of type float
* A program counter

## Instructions:

```
val,  n, m     set the memory address m to value n
load, m        load memory address m into the register
stor, m        store the register into memory address m
add,  m        add the value in memory address m to the register
div,  m        divide register by value in memory address m
mul,  m        multiply register by value in memory address m
pow,  m        raise the register to the power of memory address m
inc            increment the register by one
prt            print the register
gt,   m, i     jump to instruction i if register is greater than memory address m
jmp,  i        jump to innstruction i
```

Free hints: setting the program counter has a pitfall. The program does not use
complex numbers.
