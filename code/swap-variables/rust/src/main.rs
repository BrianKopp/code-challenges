use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Expected two command line arguments");
    }

    let mut int_one = args.get(1).unwrap().parse::<i32>().expect("Failed to parse first command line argument to int");
    let mut int_two = args.get(2).unwrap().parse::<i32>().expect("Failed to parse second command line argument to int");

    println!("Swapping out integers {:?} {:?}", int_one, int_two);
    swap_integers(&mut int_one, &mut int_two);
    println!("After swap {:?} {:?}", int_one, int_two);
}

// Make a function that swaps the integers out
// without using any temp variables
// the function handles edge cases, and otherwise
// recursively massages the integers while calling itself
// until the integer values are what it expects.
// It wants the integers to be positive, and a > b.
// and will recurse until it accomplishes that.
fn swap_integers(a: &mut i32, b: &mut i32) {
    // suppose values 5 and 3, we can swap them out by
    // updating the value of 5 to 5-3=2, then
    // updating the value of 3 to 3+2=5, then
    // updating the value of 2 to 5-2=3.

    // bail if the numbers are identical
    if *a == *b {
        return;
    }

    // check for numbers that will cause overflows
    if *a == i32::MIN {
        *a = *b;
        *b = i32::MIN;
        return;
    }

    if *b == i32::MIN {
        return swap_integers(b, a);
    }

    // check for negative signs
    if *a < 0 {
        *a *= -1;
        swap_integers(a, b);
        *b *= -1;
        return;
    }

    if *b < 0 {
        *b *= -1;
        swap_integers(a, b);
        *a *= -1;
        return;
    }

    // For simplicity, we'll require a > b
    if *b > *a {
        swap_integers(b, a);
        return;
    }

    // otherwise, things will fit nicely into this pattern
    *a = *a - *b;
    *b = *b + *a;
    *a = *b - *a;
    return;
}
