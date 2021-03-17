
const SOME_NUMBER: u32 = 100;

fn main() {
    println!("Hello, world!");
	println!("Your constant is: {}", SOME_NUMBER);

	//shadowing (a special concept so they make about it a section)
	let x = 5;
	let x = x + 1;
	let x = x * 2;
	
	println!("The values of x is: {} - should be 12", x);

	//lets do some parkour with types and shadowing uuuu rust has features
	let spaces = "    ";
	let spaces = spaces.len();

	println!("Spaces : {}", spaces);

	// this will not work with different types and mutable variables
	// YOU WILL GET A COMPILER ERROR WHEN YOU UNCOMMENT STUFF BELLOW:
	//let mut spaces = "     ";
	//spaces = spaces.len();

	// Integer types
	// i8	 u8
	// i16 	 u16
	// i32	 u32
	// i64	 u64
	// i128	 u128
	// isize usize

	// Floats
	// f32
	// f64

	// Boolean
	// bool -> true false

	// Character
	// char   (unicode)
	// 'a'
	// U+0000 -> U+D7FF
	// U+E000 -> U+10FFFF

	// Tuples (can have different type)
	// (500, 2.3, "kot")
	// let tup: (i32, f64, &str) -> typing
	// let (x, y, z) = tup; -> unpacking
	// let first = tup.0

	// Arrays (has to have same type) (stack)
	// Fixed size
	// let a = [1, 2, 3, 4, 5];
	// let a: [i32; 5] = [1, 2, 3, 4, 5]; -> type, size
	// let a = [3; 5] -> initialize array of 5 elements, all 3

	// Literals
	// Decimal  98_222 -> 98222
	// Hex		0xff
	// Octal    0o77
	// Binary	0b1111_0000
	// Byte		b'A'

	// FUN STUFF
	// when normaly you do `cargo run` this will casue for panic
	// and break the program
	// but when you run `cargo run --release` the output will be 4
	let mut bucket: u8 = 200;
	//bucket += 60;

	// THIS IS FROM RUST BOOK, WHAT IS THAT?
    //Wrap in all modes with the wrapping_* methods, such as wrapping_add
    //Return the None value if there is overflow with the checked_* methods
    //Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    //Saturate at the value's minimum or maximum values with saturating_* methods
	
	fn wrapping_add(num: u8) -> u8 {
		//return num + 60;   // you uncomment this and the problem will be there
		return num          
	}

	let output = wrapping_add(200);

	println!("The bucket is: {}", bucket);
	println!("The ouput is: {}", output);

	let tup: (i32, f64, &str) = (500, 2.3, "kot");
	let (x, y, zwierz) = tup;
	let first = tup.0;

	println!("The tuple: {:?}", tup);
	println!("The tuple unpacking: {}, {}, {}", x, y, zwierz);
	println!("The first: {}", first);

	
	let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

	let a: [i32; 5] = [1, 2, 3, 4, 5]; // type and size
	let b = [3; 5];  // array of five 3
	let march = months[2];

	println!("March is: {}", march);

	another_function();
	function_arguments(80);

	use_of_block();
	example_return();

	println!("Early return works: {}", explicit_return());

	// After if you need to have boolean - NOTHING OTHER!
	// so here we are explicit :)
	if a[1] == 2 {
		println!("Wow my first if");
	} else {
		println!("This will not be executed");
	}

	if a[2] == 2 {
		println!("nope");
	} else if a[2] == 3 {
		println!("hura");
	} else {
		println!("forget about this");
	}

	// flat/short if in expression
	let condition = true;
	let number = if condition { 5 } else { 6 };
	
	println!("The value of number is: {}", number);

	// this will fail - types are different
	// let number = if condition { 5 } else { "six" };

	// LOOPS
	loop_me_till_the_world_ends();

	// Here i can return (what?) a value from a loop
	// loop is an expresion, ok. 
	let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

	// WHILE - oldie but goldie
	let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

	// FOR - like pythons
	// acording to Rust Book, for loops increse safety :D kurwa weźcie mnie trzymajcie
	// shit, python is such a safe language :D
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

	// I'm not fucking allowing to call me Rustaceans

	// wow, i can reverse a range
	for number in (1..4).rev() {
        println!("{}!", number);
    }
}

// FUNCTIONS

fn another_function() {
	println!("Hallo");
}

// Function must have arguments types
// implicit return type is ()
fn function_arguments(x: i32) {
	println!("Dupa była {}", x);
}

// return value is last line in function AND WITHOUT THE ;
// function has to have anotated the return type
fn example_return() -> i32 {
	10
}

fn explicit_return() -> i32 {
	if true {		// this is how you fuck with the Rust compiler
					// small if and
		return 10;
	}
	20				// <- this stops being an issue :D
}

// Rust is supossedly "an expression-based language"
// uuuu
// "Other languages don't have the same distinctions" uuuu
// 
// Is Rust a smug language? fuck yeah
//
// statement does action but do not return value
// statements: let x = 6, a function definition

// expresions evaluate and give a result
// expressions: 5 + 6, calling a function, calling a macro
//			   a block is an expression
// expressions do not en with endind semicolons (DANGER DANGER)

// O KURWA :D
// so, when you have a semicolon on end - its a statement
//     when you do not have a semicolon - its an expression
// so forget this motherfucker and you return a value
// or put it and you don't return a value you wanted but ()
// this is going to make ton of fucking hard to catch runtime errors (lets check that later)
// (cause people will believe compiler, no need to use type anotations will make them
// lazy, and complication in code (which is always) hides such small things like ; )

// yeah, you can return stuff and fucking compiler wont tell, 
//		only the println macro screamed cause it can format ()
//
fn use_of_block() {
	let x = 5;

	let y = {
		let x = 3;  //x is local
		x + 1	// no semicolon! implicit return (?) this is supposedly a safe language? :D
	};

	let z = {
		x;
	};	

	println!("The Z: {:?}", z);
	println!("Lets print non local x: {}", x);
	println!("The value of y(block) is: {}", y);
}

fn loop_me_till_the_world_ends() {
	let mut x = 3;
	loop {
		println!("again!");
		
		x = x -1;
		if x == 0 {
			break
		}
	}	
}
