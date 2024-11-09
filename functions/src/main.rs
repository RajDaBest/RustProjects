/*

The main function is the entry point of many programs (by default, the entry point is main).
The fn keyword allows us to declare new functions.

Rust code uses snake case as the conventional style for function and
variable names, in which all the letters are lowercase and underscores separate words.

*/

fn main() {
    println!("Hello, world!");

    another_function();
    parameter_function(5);
    print_labeled_measurements(5, 'h');
}

fn another_function() {
    println!("Another function!");
}

/*

We define a function in Rust by entering fn followed by a function name and a set of
parentheses. The curly brackets tell the compiler where the function body begins and ends.

We can call any function we've defined by entering it's name followed by a set of parenthesis.

Note that we defined another_function after the main function in the source code; we could have defined it before as well.
Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

The lines execute in the order in which they appear in the main function (or in any other function) by default.

*/

/*

We can define functions to have parameters, which are special variables that are part of a function's
signature. When a function has parameters, you can provide it with concrete values for those
parameters. Technically, the concrete values are called arguments, but in casual conversation, people
tend to use the words parameter and argument interchangeably for either the variables in a function's definition
or the concrete values passed when you call a function.

*/

fn parameter_function(x: i32) {
    println!("The value of first parameter is: {x}");
}

/*

The declaration of parameter_function has one parameter named. The type of x is specified
as i32. When we pass 5 into the function, the println! macro puts 5 where the pair of
curly brackets containing x was in the format string.

In function signatures, you must declare the type of each parameter. This is a deliberate
decision in Rust's design: requiring type annotations in function definitions means
the compiler almost never needs you to use them elsewhere in the code to figure out what
the type you mean. The compiler is also able to give more helpful error messages if it knows
what types the function expects.

When defining multiple parameters, separate the parameter declarations with commas, like this:

*/

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*

This example creates a function named print_labeled_measurement with two parameters.
The first parameter is named value and is an i32.
The second is named unit_label and is type char.
The function then prints text containing both the value and the unit_label.

*/

/*

# Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.
Rust is an expression-based language.

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.

Creating a variable and assigning it with the let keyword is a statement.
Function definitions are also statements.

Statements do not return a value. Therefore, you can't assign a let statement to another
variable, you'll get an error:

*/

// let x = (let y = 6);

/*

The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.
In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

*/

/*

Expressions evaluate to a value and make up most of the rest of the code that you'll write
in Rust. Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value
11. Expressions can be part of statements: the 6 in the statement let y = 6; is an expression that
evaluates to the value 6.

Calling a function is an expression. Calling a macro is an expression. A new
scope block created with curly brackets is an expression:

*/

fn curly_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
}

/*

This expression:

{
    let x = 3;
    x + 1
}

is a block that, in this case, evaluates to 4. That value gets bound to y as part of the
let statement. Note that the x + 1 line doesn't have a semicolon at the end, which is unlike
most of the lines you've seen so far. Expressions do not include ending semicolons. If you add
a semicolon to the end of the expressino, you turn it into a statement, and it will then not
return a value.

A block evaluates to the last expression inside them, or the unit type () if there are none.

*/

/*

# Functions with Return Values

Functions can return values to the code that calls them. We don't name return values, but we
must declare their type after an arrow. In Rust, the return value of the function is
synonymous with the value of the final expression in the block of the body of a function. You can
return early from a function by using the return keyword and specifying a value, but most functions
return the last expression implicitly.

*/

fn five() -> i32 {
    5
}

/*

There are no function calls, macros, or even let statements in the five function - just the nujmber
5 by itself. That's a perfectly valid function in Rust. Note that the function's return type is
specified too, as -> i32.

The 5 in five is the function's return value, which is why the return type is i32.
The five function has no parameters and defines the type of the return value,
but the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to return.

*/

fn plus_one(x: i32) -> i32 {
    x + 1
}

/*

Placing a semicolon at the end of the line containing x + 1 will change it from an
expression to a statement. A function with not tail or return expression implicitly returns the unit type, ().
This will be an error since the function returns the unit type but claims to return an i32 in it's definition.
This is an error of mismatched-types.

Statements evaluate to the unit type.

*/
