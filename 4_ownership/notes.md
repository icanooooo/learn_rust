# Ownership

## What is ownership?

The set of rules of how Rust manage memory. Unlike C or C++ where you have to allocate the memory manually, or Python where you used a garbage collection, Rust use a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, it will not compile.

### Stack and Heap Quick Summary

Unlike other programming languages, Rust needs you think about the Stack and Heap more often. **Stack and Heap** are two parts of memory used in programming to store data, but they are used in very different way.

**Stack** is stores data in the memory using **LIFO (Last in, First Out)**, storing the data sequentially. Imagine a stack of plates, we can only put another plate in the top or take the top plate without it crumbling. Adding data is *Pushing onto the Stack** while removing data is *popping off the stack*. All data stored onto the stack must have a known and fixed size. In most systems, the size of stack is limited to a few megabytes (16 mb at most). Therefore, we can't store large amounts of data. If data is large in size or are not known in size at compile time, it will store it to the heap. If the data stored in the stack is to large or is a recursive function, this will cause what is called *stack overflow*.

**Heap** is less organized, you request a certain amount of space in the memory and then the memory allocator will find a spot in the heap that is large enough, marks it as being in use, and returns a *pointer*, an address to the location of the data (the *pointer* is stored in the stack). So in C/C++ we manually manage the memory, we have allocate and deallocate the memory in the heap, failure in deallocating the memory space will cause *memory leaks*, accumulating unused memory which could cause the program or system to crash. To combat this Python or Javascript, they have a *garbage collector* which is an automatic process. In rust, we use system of **Ownership & Borrowing**.

So essentially Ownership and Borrowing are rules that dictates us to write code that helps the rust compiler to catch potential memory issues, like **Dangling Pointers, data race,** and **memory leaks** at compile time.

## Ownership Rules

### Scope


