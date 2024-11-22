/*

Ownership enables rust to make memory safety guarantees without needing a garbage collector.

Ownership is a set of rules that govern how a Rust program manages memory.
All programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection that regularly look for no-longer-used memory
as the program runs; in other languages, the programmer must explicitly allocate and
free the memory. Rust uses a third approach: memory is managed through a system of
ownership with a set of rules that the compiler checks. If any of the rules are violated
, the program won't compile. None of the features of ownership will slow down your program
while it's running.

*/

fn main() {}
