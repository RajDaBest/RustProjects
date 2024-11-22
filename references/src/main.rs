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

fn calculate_length(s: &String) -> usize { // s is a reference to a string
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



*/
