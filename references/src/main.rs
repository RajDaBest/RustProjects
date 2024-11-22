/*

References and Borrowing

The issue with the tuple code previously is that we have to return the String to the calling
function so we can still use the String after the call to calculate_length, because the
String was moved into calculate_length. Instead, we can provide a reference to the String value.
A reference is like a pointer in that it's an address we can follow to access the data
stored at that address; that data is owned by some other variable. Unlike a pointer, a
reference is guaranteed to point to a valid value of a particular type for the life
of that reference.

*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is: {len}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

/*

We pass &s1 into calculate_length and, in it's definition, we take &String rather than
String. These ampersands represent references, and they allow you to refer to some
value without taking ownership of it.

The opposite of referencing by using & is dereferencing, which is accomplished with the
dereference operator, *.

The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
Because it does not own it, the value it points to will not be dropped when the reference
stops being used.

Likewise, the signature of the function uses & to indicate that the type of the parameter
s is a reference.

The scope in which the variable s is valid is the same as any function
parameter's scope, but the value pointed to by the reference is not
dropped when s stops being used, because s doesn't have ownership.

When functions have references as parameters instead of actual values
, we won't need to return the values in order to give back ownership, because
we never had ownership.

We call the action of creating a reference borrowing. Just as variables
are immutable by default, so are references. We're not allowed to
modify something we have a reference to.

*/

/*

Mutable References

We can allow a borrowed value to be modified using a mutable reference. For
a variable to have a mutable reference, it must first be mutable itself.

*/

fn _mutable_ref() -> () {
    let mut s = String::from("hello");

    _change(&mut s);
}

fn _change(some_string: &mut String) -> () {
    some_string.push_str(", world");
}

/*

Mutable references have one big restriction: if you have a mutable
reference to a value, you can have no other references to that value.
This code that attempts to create two mutable references to s will fail:

*/

fn _dup_ref() -> () {
    let mut _s: String = String::from("hello");

    let _r1: &mut String = &mut _s;
    // let _r2: &mut String = &mut _s;

    // println!("{_r1}, {_r2}");
}

/*

This code is invalid because we cannot borrow s as mutable more than
once at a time. The first mutable borrow is in r1 and must last until it's used
in the println!, but between the creation of that mutable reference and it's usage,
we tried to create another mutable reference in r2 that borrows the same
data as r1.


The restriction preventing multiple mutable references to the same data
at the same time allows for mutation but in a very controlled fashion.
It's something new Rustaceans struggle with because most languages let
you mutate whenever you'd like. The benefit of having this restriction
is that Rust can prevent data races at compile time. A data race is
similar to a race condition and happens when these three behaviours occur:

1. Two or more pointers access the same data at the same time
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose
and fix when you're trying to
track them down at runtime.

As always, we can use curly brackets to create a new scope,
allowing for multiple mutable references, just not simultaneous ones:

*/

fn _mut_ref_dup() -> () {
    let mut _s: String = String::from("hello");

    {
        let _r1: &mut String = &mut _s;
    } // r1 goes out of scope here, so we can make a new reference with
      // no problem

    let _r2: &mut String = &mut _s;
}

/*

Rust enforces a similar rule for
combining mutable and immutable references.

We also cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don’t expect the value to suddenly
change out from under them!
However, multiple immutable references are allowed because
no one who is just reading the data has the ability to affect
anyone else’s reading of the data.

Note that a reference’s scope starts from where it is
introduced and continues through the last time that
reference is used (or when it's block scope ends, whichever comes first).
For instance,
this code will compile because the last usage of
the immutable references, the println!, occurs
before the mutable reference is introduced:

*/

fn _let_go() -> () {
    let mut _s: String = String::from("hello");

    let _r1: &String = &_s; // no problem
    let _r2: &String = &_s; // no problem

    // variables r1 and r2 must not be used after this point

    let _r3: &mut String = &mut _s;
    println!("{_r3}");
}

/*  

The scopes of the immutable references r1 and r2 end after the 
println! where they are last used, which is before the 
mutable reference r3 is created. These scopes don’t overlap, so this 
code is allowed: the compiler can tell that the reference is no 
longer being used at a point before the end of the scope.

*/
