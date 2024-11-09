use std::io;

fn main() {
    let fibo_n: i32 = loop {
        println!("Enter n:");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Enter a valid number");

        let n: i32 = match n.trim().parse() {
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
            Ok(num) => num,
        };

        if n < 0 {
            println!("Enter a non-negative integer");
        } else {
            break n;
        }
    };

    if fibo_n == 0 {
        println!("{fibo_n}th fibonacci number is 0");
    } else if fibo_n == 1 {
        println!("{fibo_n}th fibonacci number is 1");
    } else {
        let mut a1: u32 = 0;
        let mut a2: u32 = 1;
        let mut count: i32 = fibo_n - 2;
        let mut temp: u32 = 0;

        while count > 0 {
            temp = a2;
            a2 = a1 + a2;
            a1 = temp;
            count -= 1;
        }

        println!("The {fibo_n}th fibonacci number is: {a2}");
    }
}
