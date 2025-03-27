use std::io;

fn main() {
    let mut inp = String::new();
    println!("Type in the nth number of the fibonacci sequence you want");

    io::stdin().read_line(&mut inp).expect("Failed to read line");
    let n: u64 = inp.trim().parse().expect("Input not an integer");

    let fib_calc = fibonacci(n);
    println!("fibonacci number at {n}th position is {fib_calc}");

}

fn fibonacci(x: u64) -> u64 {
    if x <= 0 {
        return 0;
    } else if x <=1 {
        return 1;
    }
    fibonacci(x - 1) + fibonacci(x - 2)
}
