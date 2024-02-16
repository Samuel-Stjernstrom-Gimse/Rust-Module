use std::io;

fn main() {

    let x: i8 = 9;

    let y: i8 = 3;

    let z: i8 = x + y;



    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("{}", input);






}