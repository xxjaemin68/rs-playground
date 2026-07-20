// xxjaemin68 — messing with basic rust patterns

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("usage: scratch <n>");
        std::process::exit(1);
    }

    let n: u64 = args[0].parse().expect("expected an integer");
    let f = fib(n);
    println!("fib({n}) = {f}");
}

fn fib(n: u64) -> u64 {
    if n < 2 { return n; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n - 1 {
        let t = a + b;
        a = b;
        b = t;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn zero() { assert_eq!(fib(0), 0); }
    #[test] fn one()  { assert_eq!(fib(1), 1); }
    #[test] fn ten()  { assert_eq!(fib(10), 55); }
}
