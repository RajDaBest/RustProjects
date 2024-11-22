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

Ownership Rules:

1. Each value in Rust has an owner
2. There can be only one owner at a time
3. When the owner goes out of scope, the value will be dropped

A scope is the range within a program for which an item is valid. A variable is valid
from the point at which it's declared until the end of the current scope.

*/

fn _scope() -> () {
    {
        // s is not valid here, it's not declared yet
        let _s: &str = "hello"; // s is valid from this point forward

        // do stuff with s
    } // the scope is now over, and s is no longer valid

    /*

    When s comes into scope, it is valid and it remains valid until it goes out of scope.

    */
}

/*

The String Type

The types covered previously are of a known size, can be stored on the stack and
popped off the stack when their scope is over, and can be quickly and trivially
be copied to make a new, independent instance if another part of code needs to use the
same value in a different scope. But we want to look at data that is stored on the heap
and explore how Rust knows when to clean up that data, and the
String type is a great example.

We’ll concentrate on the parts of String that relate to ownership.
These aspects also apply to other complex data types,
whether they are provided by the standard library or created by you.

We’ve already seen string literals, where a string value is hardcoded into our program.
String literals are convenient, but they aren’t suitable for every situation in which we may want to use text.
One reason is that they’re immutable.
Another is that not every string value can be known when we write our code:
for example, what if we want to take user input and store it?
For these situations, Rust has a second string type, String.
This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
You can create a String from a string literal using the from function, like so:

*/

fn _string_literal() -> () {
    let _s: String = String::from("hello");

    // The double colon operator allows us to namespace this particular from function
    // under the String type rather than using some sort of name like string_from.

    // This type of string can be mutated:

    let mut _s: String = String::from("hello, world");
}

/*

Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the text is
hardcoded directly into the final executable. This is why string literals are fast and
efficient. But these properties only come from the string literal's immutability.
Unfortunately, we can't put a blob of memory into the binary for each piece of text
whose size is unknown at compile time and whose size might change while running the program.

With the String type, in order to support a mutable, growable piece of text, we need to
allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
This means:

1. The memory must be requested from the memory allocator at runtime.
2. We need a way of returning this memory to the allocator when we're done with our String.

That first part is done by us: when we call String::from, it's implementation requests the
memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC
keeps track of and cleans up memory that isn't being used anymore, and we don't need to
think about it. In most languages without a GC, it's our responsibility to identify
when memory is no longer being used and to call code to explicitly free it, just as we
did to request it. Doing this correctly has historically been a difficult programming problem.
If we forget, we’ll waste memory.
If we do it too early, we’ll have an invalid variable.
If we do it twice, that’s a bug too.
We need to pair exactly one allocate with exactly one free.

Rust takes a different path: the memory is automatically returned once the variable
that owns it goes out of scope.

*/

fn _memory_one() -> () {
    {
        let _s: String = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is no longer valid, and s is no longer valid
}

/*

There is a natural point at which we can return the memory our String needs to the allocator:
when s goes out of scope. When a variable goes out of scope, Rust calls a special function
for us. This function is called drop, and it's where the author of String can put the
code to return the memory. Rust calls drop automatically at the closing curly bracket.

This pattern has a profound impact on the way Rust code is written.
It may seem simple right now, but the behavior of code can be unexpected in more complicated situations
when we want to have multiple variables use the data we’ve allocated on the heap.

*/

/*

Variables and Data interacting with Move

Multiple variables can interact with the same data in different ways in Rust.

*/

fn _interact_integer() -> () {
    let _x = 5;
    let _y = _x;

    // bind the value 5 to x; then make a copy of the value in x and bind it to y
    // we then have two variables, x and y, and both equal 5

    // This is indeed what is happening, because integers are simple values with a known,
    // fixed size, and these two 5 values are pushed onto the stack.

    let _s1 = String::from("hello");
    let _s2 = _s1;

    // This looks very similar, so we might assume that the way it works
    // would be the same: that is, the second line would make a copy of the value in s1
    // and bind it to s2. But this isn't quite what happens.
}

/*  

A String is made up of three parts: a pointer to the memory that holds the contents of
the string, a length, and a capacity. This group of data is stored on the stack.
The length is how much memory, in bytes, the contents of the String are currently using. The
capacity is the total amount of memory, in bytes, that the String has received from the allocator.
The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length
, and the capacity that are on the stack. We don't copy the data on the heap that the
pointer refers to. If Rust instead copied the heap data as well, the operation s2 = s1 
could be very expensive in terms of runtime performance if the data on the heap were large.

Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function
and cleans up the heap memory for that variable.

This is a problem: when s2 and s1 go out of scope, they will both try to free the same 
heap memory. This is known as a double free error and is one of the memory safety bugs.
Freeing memory twice can lead to memory corruption, which can potentially lead to
security vulnerabilities.

*/

fn main() {}
