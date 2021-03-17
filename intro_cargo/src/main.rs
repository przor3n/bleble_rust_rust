use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

	loop {  // start a loop
    	let mut guess = String::new(); // new String mutable

	
    	io::stdin()
        	.read_line(&mut guess)     // pass variable by reference, (enter is added to the end of the value)
        	.expect("Failed to read line");  // returns Result
									     // Result is an enum 
										 // Result is Ok or Err
    	println!("You guessed: {}", guess);  // String placeholders

		let x = 5;
		let y = 10;

		println!("x = {} and y = {}", x, y);

		// we shadow the previous variable and parse the previous to unsign int 32
		let guess: u32 = match guess.trim().parse() {   // we can match Result of parse()
			Ok(num) => num,							    // crashing with expect change with match
			Err(_)  => continue,
		}

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => { 
				println!("You win!");
				break
			}
		}
	}
}
