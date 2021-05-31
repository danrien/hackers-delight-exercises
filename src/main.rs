use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let name = &args[1];
    println!("Hello, {}!", name);
    println!("I have a program to show you {}. It will given the next higher number after the passed in number that has the same number of 1-bits.", name);

    let x: u16 = 0b1000_0000_1111_0000;
    println!("For example, imagine I pass in 0b{:16b}. The next higher number with the same number of 1-bits is 0b{:16b}", x, snoob(x));
}

fn snoob(x: u16) -> u16 {
    let smallest = x & (-(x as i16) as u16);
    let ripple = x + smallest;
    let mut ones = x ^ ripple;
    ones = (ones >> 2) / smallest;
    return ripple | ones;
}