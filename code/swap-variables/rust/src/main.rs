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
fn swap_integers(a: &mut i32, b: &mut i32) {
    // suppose values 5 and 3, we can swap them out by
    // updating the value of 5 to 5-3=2, then
    // updating the value of 3 to 3+2=5, then
    // updating the value of 2 to 5-2=3.

    // some limitations - this won't protect against integer overflows
    // e.g. if a is int.min and b is int.max, bad things are going to happen
    
    // quickly handle the case where we have zeros
    if *a == 0 || *b == 0 {
        *a = *a - *b;
        *b = *b + *a;
        *a = *b - *a;
        return;
    }

    // handle the case where the numbers have the same sign
    if (*a)/(*b) >= 0 {
        if (*a < 0 && *a < *b) || (*a > 0 && *a > *b) {
            // do a-b
            *a = *a - *b;
            *b = *b + *a;
            *a = *b - *a;
            return;
        } else {
            // do b-a
            *b = *b - *a;
            *a = *a + *b;
            *b = *a - *b;
            return;
        }
    } else {
        // they have different signs
        if (*a < 0 && *a == i32::MIN) || (*a < 0 && -1*(*a) > *b) {
            // absolute value of a is greater than b
            // since different signs, swap the add/subtracts
            println!("block one {:?} {:?}", a, b);
            *b = *b + *a;
            println!("{:?} {:?}", a, b);
            *a = *b - *a;
            println!("{:?} {:?}", a, b);
            *b = *b - *a;
            println!("{:?} {:?}", a, b);
            return;
        } else if (false) {
            // handle the case where b should be done first
        //  || (*a > 0 && *b != i32::MIN && *a > -1*(*b))
        } else {
            // do b-a
            println!("block two {:?} {:?}", a, b);
            *b = *b + *a;
            println!("{:?} {:?}", a, b);
            *a = *b - *a;
            println!("{:?} {:?}", a, b);
            *b = *a + *b;
            println!("{:?} {:?}", a, b);
            return;
        }
    }
}
