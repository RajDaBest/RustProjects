fn main() {
    // rust is a statically typed language, which means that it must know the types
    // of all variables at compile time

    // the compiler can usually infer what type we want to use based on the
    // value and how we use it. In the cases where many types are possible, such as
    // when we converted a string to a numeric type using parse, we must add a type
    // annotation, like this:

    let guess: u32 = "42".parse().expect("Not a number!");

    // if we don't add the : u32 type annotation, rust will display an error which shows
    // that the compiler needs more information from us to know which type we want to use

    // isize and usize type size depends on the architecture of the computer your program
    // is running on: 64 bits if you're on a 64-bit architecture and 32-bits if you are on a
    // 32-bit architecture

    // number literals that can be multiple numeric types allow a type suffix, such as 57u8, to
    // designate the type; the default is i32 for integer types

    // the primary situation in which you'd use isize or usize is when indexing some sort of collection

    // the default rust type for floating-point values is f64

    // char literals are specified with single quotes, as opposed to string literals, which use double quotes

    // rust's char type is four bytes in size and represents a Unicode Scalar Value, which means
    // it can store more than just ASCII.

    // Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

    /*

    # Compound Types

    Compound types can group multiple values into one type. Rust has two primitive compound types: list and tuples

    ## The Tuple Type

    A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    We create a tuple by writing a comma-separated list of values inside parentheses.
    Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    We’ve added optional type annotations in this example:

    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    /*

    The variable tup binds to the entire tuple because a tuple is considered a single compound
    element. To get the individual elements of a tuple, we can use pattern matching to destructure
    a tuple value, like this:

    */

    let (x, y, z) = tup;

    println!("The value of y is {y}");

    /*

    Pattern matching is used with let to take tup and turn it into three separate variables, x, y, and z.
    This is called destructuring because it breaks the single tuple into three parts.

    We can also access a tuple element directly by using a period (.) followed by the index
    we want to access. Tuples are zero-indexed.

    */

    let m = tup.0;

    /*

    The tuple without any values has a special name, unit. This value and it's corresponding
    type are both written () and represent an empty value or an empty return value. Expressions
    implicitly return the unit value if they don't return any other value.

    */

    let unit_tup: () = ();
}
