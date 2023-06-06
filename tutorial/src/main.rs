fn main() {
    println!("Hello, world!");

    let x: u32 = 4;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x);
    }
    let x = x + 1;
    println!("x is: {}", x);
}
