fn main() {
    /*

    The most common constructs that let you control the flow of execution of Rust code
    are if expressions and loops.

    # if Expressions

    All if expressions start with the keyword if, followed by a condition.
    We place the block of code to execute if the condition is true immediately
    after the condition inside curly brackets. Blocks of code associated with the conditions
    in if expressions are sometimes called arms, just like the arms in match expressions.

    Optionally, we can also include an else expression to give the program an alternative block
    of code to execute should the condition evaluate to false. If you don't provide an else
    expression and the condition is false, the program will just skip the if block and move
    on to the next bit of code.

    The condition MUST be an expression of bool type (or evaluate to a bool type); if it isn't, we will get an error.

    */

    /*

    Because if is an expression, we can use it on the right side of a let statement to assign the
    outcome to a variable:

    */

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    /*

    The number variable will be bound to a value based on the outcome of the if expression.

    Remember that blocks of code evaluate to the last expression in them, and numbers by themselves
    are also expressions.
    In this case, the value of the whole if expression depends on which block of code executes.
    This means the values that have the potential to be results from each arm of the if must be the same type; in the previous example,
    the results of both the if arm and the else arm were i32 integers.
    If the types are mismatched, as in the following example, we’ll get an error:

    */

    let number_two = if condition { 5 } else { "six" };

    /*

    When we try to compile this code, we'll get an error. The if and else arms have value
    types that are incompatible.

    The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a string.
    This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively.
    Knowing the type of number lets the compiler verify the type is valid everywhere we use number.
    Rust wouldn’t be able to do that if the type of number was only determined at runtime;
    the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

    */

    /*

    # Repetition with Loops

    ## The loop keyword

    The loop keyword tells Rust to execute a block of code over and over again
    forever until you explicitly tell it to stop.

    You can place the break keyword in any position in a loop to tell the program to
    immediately exit the loop; you can also use the continue keyword to skip over any
    of the remaining code in the current loop iteration.

    One of the uses of loop is to retry an operation you know might fail, such as
    checking whether a thread has completed it's job. You might also need to pass the
    result of that operation out of the loop to the rest of the code. To do this, you can add
    the value you want returned after the break expression you use to stop the loop; that value
    will be returned out of the loop so you can use it:

    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    /*

    If you have loops within loops, break and continue apply to the innermost loop at that
    point. You can optionally specify a loop label on a loop that you can then use with break
    or continue to specify that those keywords apply to the labeled loop instead of the innermost
    loop. Loop labels MUST begin with a single quote:

    */

    let mut count = 0;

    'couting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    /*

    ## Conditional loops with while

    while (condition)
    {
        // code block
    }

    While condition evaluates to true, the code runs; otherwise, it exits the loop

    */

    /*

    ## Looping through a Collection with for

    You can use the while construct to loop over the elements of a collection, such as an array:

    */

    let collection: [i32; 32] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while (index < 5) {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    /*

    Even though index will reach a value of 5 at some point, the loop stops executing
    before trying to fetch a sixth value from this array.

    This approach is slow, because the compiler adds runtime code to perform the
    conditional check of whether the index is within the bounds of the array
    on each iteration through the loop.

    As a more concise alternative, you can use a for loop and execute some code
    for each item in a collection:

    */

    for element in a {
        println!("the value is {element}");
    }

    /*

    Range, provided by the standard library, generates all numbers in sequence
    starting from one number and ending before another number.

    In the following loop, we use a method rev() to reverse the range:

    */

    for elt in (1..4).rev() {
        println!("{number}");
    }
}
