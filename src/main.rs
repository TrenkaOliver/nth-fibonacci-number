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
    let n = n.abs();
    if n < 2 {
        return n;
    } else {
        let mut p2 = 0;
        let mut p1 = 1;
        let mut current = 0;

        for _ in 2..=n {
            current = p1 + p2;
            p2 = p1;
            p1 = current;
        }

        return current;
    }
}
