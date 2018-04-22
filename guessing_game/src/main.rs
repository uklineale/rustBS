extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1,101);

	loop {
		println!("What's your number?");
		
		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(um) => um,
			Err(_) => continue,
		};

		println!("You guessed {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
