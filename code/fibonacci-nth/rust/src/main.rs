use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_string = args.get(1).expect("expected a command line argument");
    let n = n_string.parse::<i32>().unwrap();
    let fib = nth_fibonacci(n).unwrap();
    println!("{:?}-th fibonacci number: {:?}", n, fib);
}

fn nth_fibonacci(n: i32) -> Result<i32, &'static str> {
    let mut a = 0;
    let mut b = 1;
    let mut count = 1;

    if n <= 0 {
        return Err("n-th fibonacci number must be positive");
    }
    if n == 1 {
        return Ok(1);
    }

    loop {
        let fib = a + b;
        a = b;
        b = fib;
        count += 1;
        if count == n {
            return Ok(fib);
        }
    }
}