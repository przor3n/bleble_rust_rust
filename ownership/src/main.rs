// stack = last in, first out
// stack has data fixed size
// stack is fast
// basic types are on stack

// heap = anywhere, you get space and a pointer to it
// heap has data unknown size or size that changes
// heap is slow
// complex data types are on heap

// pointer is a value

// when you call a function
// its arguments (values and pointers) and local variables
// are pushed on stack,
// when function is over,
// the data is poped off the stack

//    Each value in Rust has a variable that’s called its owner.
//    There can only be one owner at a time.
//    When the owner goes out of scope, the value will be dropped.

// Scope is to the block

// Code moves out of the block/scope, variable dies (fuck me if this isnt garbage collector)
// so, there is a special function `drop` 
// and "Rust" is calling it to free the memory.
// turns out (for now) that you can put stuff in String code
// and override `drop` function 

// also, why the fuck they didn't explain mechanics of a programing languag
// how stuff is done (top down) how scopes work (ok, ownership tells that but)
// and how to organize files and code? how to test?

// so, when 

fn main() {
    println!("Hello, world!");

	{
		let mut s = String::from("hello");
		s.push_str(", world!"); // push_str() appends a literal to a String
    	println!("{}", s); // This will print `hello, world!`
	}   // s is no longer with us


	// so we allocate memory, and then attach a pointer
    let s1 = String::from("hello");
	// here rust is making s1 obsolete and no longer having
	// reference to the place in the memory
    let s2 = s1;

	// this will cause an error
    // println!("{}, world!", s1); 

	// this will work: 
	let s1 = String::from("hello");
    let s2 = s1.clone();  // you are making a clone
	// and here Rust makes you do stuff explicit

    println!("s1 = {}, s2 = {}", s1, s2);


	// if this is done on simple types
	// you can do it.
 	let x = 5;
    let y = x;   // this is another place for mistakes
	// and here Rust is implicit
	// we still can call clone() ! 
    println!("x = {}, y = {}", x, y);

	// Łoł, there is a "trait" Copy
	// when you use this trait on a variable
    // you will be able to do cloning without cloning
	// great.
	
	// What has the Copy trait:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

	// Fuck. Comments in Rust suck donkey dick.


	// Another stuff
	let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
	
	// This will fail as I "borrow the value after move"
	// println!("Is s is back? :{}", s);
    
	let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
	
	// This is possible because primitives are "copied"	
	println!("I can use Integer {}", x);

	// So returning a value gives you ownership
	let s1 = gives_ownership();

	let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2); 	// i can take it and return it as something else
	// this is utter fuckery, it makes sens yes, 
	// you read the code and dont think about what the fuck happend there
	// but fuck, i like the idea that i pass something and then i still can
	// use it. but ok. it's cool. but on the other hand, 
	// if people took a variable changed its value and then worked on it
	// its their fucking problem. 

	// the line below is an error 
	// println!("This is an error {}", s2);
	println!("This is not {}", s3);


	// REFERENCES

	let mut s1 = String::from("hello");

	// I pass stuff with reference
    let len = calculate_length(&s1);

	// ok, i can fuck with the value :D
	lets_fuck_with_its_value(&mut s1);
	lets_fuck_with_its_value(&mut s1);
	
	// with references i pass stuff to function
	// and keep the value in this scope    
	println!("The length of '{}' is {}.", s1, len);
	
	// You cant keep more than one mutable reference
	// let r1 = &mut s1;
	// let r2 = &mut s2;
	// is that a problem??
	// this is suposedly saving me from data race
    // so Rust is so super it forbids me to make data races
	// but that's bullshit. i can have pointer data races only (isnt that a poor design?)

	//oh this is possible
	{ 
		let r1 = &mut s1;
    }
    let r2 = &mut s1;

	// i cannot have immutable and mutable references (i can have many immutable)

	// sooo this is enforcing of good style, ok

	// here is stuff interesting:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // <- i put here stuff out of scope
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3); 

	// dangling references are not possible to do. ok.
    // dangling reference is when you create a variable and the return reference to it
	// and the variable goes out of scope.

	// SLICES

	let s = String::from("hello world");
	
	// slices with single words
    let hello = &s[0..5];
    let world = &s[6..11];
	
	println!("So {} {}", hello, world);

	let s = String::from("hello");

	// simpler slices up to 2 eleemnt
	let slice = &s[0..2];
	let slice = &s[..2];

	let s = String::from("hello");

	let len = s.len();

	// simpler slices to the end
	let slice = &s[3..len];
	let slice = &s[3..];

	let s = String::from("hello");

	let len = s.len();

	// whole string as slices
	let slice = &s[0..len];
	let slice = &s[..];

}

fn lets_fuck_with_its_value(s: &mut String) {
	s.push_str(" ola");
}

fn calculate_length(s: &String) -> usize {
	return s.len();
}

fn gives_ownership() -> String {
	let some_string = String::from("DUpa");
	return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
	return a_string;
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

/*

A test
*/
