use std::io;

fn main() {
    println!("Let's find the nth fibonacci number!");
    println!("Enter n's value");

    let mut n = String::new();
    let n = loop {
        io::stdin().read_line(&mut n).expect("Error reading the input");
        match n.trim().parse::<isize>() {
            Ok(num) => break num,
            Err(_) => println!("Enter a whole number"),
        };
    };

    let value = f(n);
    if n < 0 && n % 2 == 0 {
        println!("-{value}");
    }
    else {
        println!("{value}");
    }

}

fn f(n: isize) -> isize {
    if n <= 1 {
        n
    } else {
        f(n - 1) + f(n - 2)
    }
}
