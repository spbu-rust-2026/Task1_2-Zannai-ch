use std::io;
use std::io::Read;
fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let mut nekusok = input.split_whitespace();
	let mut sum: i128 = 0;
	loop {
		let ch = match nekusok.next() {
			Some(x) => x,
			None => break,
		};
		let n: i128 = match ch.parse() {
			Ok(n) => n,
			Err(_) => {
				println!("NaN");
				return;
			}
		};
		if n == -1 {
			break;
		}
		if n <= 0 {
			println!("NaN");
			return;
		}
		sum+=n;
	}
	println!("{}", sum)
}
