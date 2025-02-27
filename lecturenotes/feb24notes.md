# What is a system call?
1. Program
    - Process (abstraction process)

2. OS boot time -> Trap table: an instruction that can only be executed in kernel mode.
    - User mode
    - Kernel mode

Trap table sets up a system call that leads to a function pointer, which calls the system during that process.
    1. Write 
    2. Read

CPU: all the registers deal with low-level language, programming is high-language therefore they are compiled to be compatible.
    - Each process has it's own stack (kernel stack)

Limited Direct Execution (LDE) - formal execution how the system call works

System call = assembly instruction (populate registers with correct value, controlled by kernel)
