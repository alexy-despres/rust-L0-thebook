fn main() {
    let x = 5;

    let x = x + 1; // Here we shadow the previous binding of x

    {
        let x = x * 2; // And here we shadow it again but within an inner scope
        println!("The value of x in the inner scope is: {x}"); // So this would output 12
    }

    println!("The value of x is: {x}"); // While this would output 6
}
