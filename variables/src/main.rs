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

fn main() {
    let x = 5;
    println!("Initial value of x: {x}");

    let x = x + 1;

    // first variable is shadowed by the second variable, which means that the
    // second variable is what the compiler will see when you use the name of the variable.
    // in effect, the second variable overshadows the first, taking any further uses of the variable name
    // to itself until either it is overshadowed or the overshadowing scope ends

    println!("Second value of x: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The final value of x: {x}");
}
