use std::env;

// Expect two integers
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("expected two command line arguments");
    }
    let a = args.get(1).unwrap().parse::<i32>().expect("expected command line args to be int");
    let b = args.get(2).unwrap().parse::<i32>().expect("expected command line args to be int");

    let off_by_one = int_diff_by_one_bit(a, b);
    println!("Ints {:?} and {:?} are off by one bit? {:?}", a, b, off_by_one);
}

fn int_diff_by_one_bit(a: i32, b: i32) -> bool {
    let mut diff = a^b;
    let mut found_diff_bit = false;

    while diff > 0 {
        if diff&1 == 1 {
            if found_diff_bit {
                // we have already found a diff bit,
                // thus, there are 2+ diff bits
                return false;
            } else {
                found_diff_bit = true;
            }
        }

        diff = diff >> 1;
    }

    found_diff_bit
}