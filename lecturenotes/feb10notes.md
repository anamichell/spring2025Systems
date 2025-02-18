# Rust Key Concepts
- Ownership 
- Borrowing

## General Cycle (Logic)
1. Write a program
2. Compile 
3. See results

Next level: Need to take into account when program connects to hardware, the operating system.

## System Call
- OS Kernel

Limited Direct Execution - program cannot access hardware, but kernel has unlimited access. Every program executed from program (user mode) [0], and executed from Kernel is kernel mode. Kept track by CPU [1].
- MMM performs calculation to make sure the memory is switched correctly. 

## Abstraction of
**Memory Visualization**
PBC - 

## Philosophy of Computing (Seed Idea)
1. Computation
2. Memory Management

Page swap - swapping memory 

## Programming Languages
1. Believe in garbage collector: create a prog. language and include another prg. garbage collector. As soon as memory is not used it is thrown (does not swap). Ex: Java, Go, Swift
- Con: Always running

2. Manual memory management: asking the system for memory. Ex. C, C++
- Easy to ask for memory, but When to give memory back?: 

Data Rays
Double free

