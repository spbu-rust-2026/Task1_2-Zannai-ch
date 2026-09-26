use std::io;
use std::io::Read;
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut kusok = input.split_whitespace();
    let x: i64 = match kusok.next().unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("NaN");
            return;
        }
    };
    let y: i64 = match kusok.next().unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("NaN");
            return;
        }
    };
    println!("{}", x + y)
}
