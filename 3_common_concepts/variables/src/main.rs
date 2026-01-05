const TWO: u8 = 1 + 1;
// let y = 7; <- This would not work as you can't declare a variable outside of a function

fn main() {
    let mut x = 5; // Here we use the mut keyword to make it mutable.
    println!("The value of x is: {x}");
    x = 6; // Without the use of the mut keyword, we wouldn't have been able to do this.
    println!("The value of x is: {x}");
    println!("1 + 1 = {TWO}")
}
