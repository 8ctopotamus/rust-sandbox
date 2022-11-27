// /book/ch03-01-variables-and-mutability.html

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3hrs = {THREE_HOURS_IN_SECONDS}s");

    // note: can't assign diff types
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
