const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// constants are always immutable and this can't be changed unlike variables
// type of the constant must always be annotated

fn main() {
    let mut x = 5;
    // variables are immutable by default but can be converted into mutable ones by using the mut keyword
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x now is {x}");
}
