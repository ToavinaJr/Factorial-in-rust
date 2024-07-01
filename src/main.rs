mod factorial;
use std::io;

fn main() {
    let mut n = String::new();
    println!("Entrer un nombre entier positif");
    io::stdin()
        .read_line(&mut n)
        .expect("Invalid input");
    let n : u64 = n.trim().parse().expect("INvalid cast");
    println!("{}! = {}", n, factorial::factorial(n));
}
