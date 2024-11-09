/* const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// constants are always immutable and this can't be changed unlike variables
// type of the constant must always be annotated

fn main() {
    let mut x = 5;
    // variables are immutable by default but can be converted into mutable ones by using the mut keyword
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x now is {x}");
}
 */

/*
fn main() {
    let x = 5;
    println!("Initial value of x: {x}");

    let x = x + 1;

    // first variable is shadowed by the second variable, which means that the
    // second variable is what the compiler will see when you use the name of the variable.
    // in effect, the second variable overshadows the first, taking any further uses of the variable name
    // to itself until either it is overshadowed or the overshadowing scope ends

    // shadowing is different from making the variable mutable because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    println!("Second value of x: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The final value of x: {x}");
} */

fn main() {
    // the other difference between mut and shadowing is that because we're effectively creating
    // a new variable when we use the let keyword again, we can change the type of the value but reuse the
    // same name. so this is allowed:

    let spaces = "   ";
    let spaces = spaces.len();

    // the first spaces is a string type and the second spaces is a number type

    // the following however is an error:

    let mut var = "   "
    var = var.len();

    // the error says we are not allowed to mutate a variable's type
}
