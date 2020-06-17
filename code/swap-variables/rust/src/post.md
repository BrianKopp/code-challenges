Hi there! I'm back with another rust coding challenge!

Problem Statement
Write a function that takes two integers and swaps their values. No temporary variables are allowed!

Implementation

Let's get to work. Normally, this would be an easy task, because we'd just take variable c as a temporary placeholder for integers a and b. But since we can't use any temporary variables, we have to just use the variables a & b. So how can we do that? I also didn't mention anything about returning or overwriting the original two integer values, so I'm guessing I need to modify the original integer values from within the function. This is a great example to test out ownership and memory access using rust.

Get started by calling cargo new swap-variables and opening your favorite text editor to src/main.rs.

So how can we do this? If we have integer a and integer b, how can we swap their values. Something like this is no good:

Rust
fn swap_integers(a: i32, b: i32) {
    // a is 5, b is 3
    a = b; // a is now 3
    b = a; // b is now 3, sad!
}
EnlighterJS Syntax Highlighter
Instead, what we could do is use some arithmetic to store information during the swap. If we have two integers, a=5,b=3, we can take the subtraction of a-b=5-3=2. Instead of setting a=b in the swap, we could set a's value to the difference. Now we have a=2,b=3. We want b=5 in the end, so we can set b=b+a=3+2=5! Now we still have a=2 which is no good, but we know we want a=3, so we can say a=b-a=5-2=3. 

Basically instead of having each variable store its value, we set one variable to store the difference between the two values. Now we can swap the variable's values without losing information! Something like:

Rust
fn swap_integers(a: i32, b: i32) {
    a = a - b;
    b = b + a;
    a = b - a;
}
EnlighterJS Syntax Highlighter
But we still need to swap out the original variables in place, so let's change the function arguments to use references. We need to make these mut variables since we'll be mutating them.

Rust
fn swap_integers(a: &mut i32, b: &mut i32) {
    *a = *a - *b;
    *b = *b + *a;
    *a = *b - *a;
}
EnlighterJS Syntax Highlighter
Nice! We pass in the reference to the i32 variables, which means we can modify their values in place. We use the dereference * symbol to get and set the value from the reference. The compiler forces us to make the variables mut since we're mutating their values. We don't own these variables, so the compiler forces the caller to acknowledge that we'll be mutating them.

Next, we need to invoke this function. I'll update the main function like so:

Rust
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
EnlighterJS Syntax Highlighter
Running cargo run 5, 3 yields the following output:

Generic Highlighting
Swapping out integers 5 3
After swap 3 5
EnlighterJS Syntax Highlighter
Not Perfect
This works for most cases, but not for all. When you're doing add/subtract operations with integral values, you should always think about overflows. What happens if a is a really super negative number, close to i32.MIN? And then what if b is a really super huge number, close to i32.MAX? You can't really go much lower than i32.MIN, so if you try to subtract i32.MAX, you're going to have an overflow. In fact, rust catches this and explodes!

You need to run ./target/debug/swap-variables as the command instead of cargo run in order to successfully pass in a negative integer on the command line. So... ./target/debug/swap-variables -2147483648 2100000000 results in:

Generic Highlighting
Swapping out integers -2147483648 2100000000
thread 'main' panicked at 'attempt to subtract with overflow', src/main.rs:29:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
EnlighterJS Syntax Highlighter
How cool is that?!

We can add in some logic in here to control who gets subtracted from whom.

