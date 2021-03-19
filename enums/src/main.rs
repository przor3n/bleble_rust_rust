#[derive(Debug)]
enum vierte {
    v4,
    v6,
}

#[derive(Debug)]
enum sehste {
    V4(String), // i can have here V4(u8, u8, u8)
    V6(String),
}

#[derive(Debug)]
struct Address {
    kind: vierte,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Dupa: {:?}", self)
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Hawaii,
    Tenessy,
    Oklahoma,
    Texas,
    NewMexico,
    LosAngeles,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }               //check the fact of missing "," -> it can be or be gone
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    println!("Hello, world!");

    let four = vierte::v4;
    let six = vierte::v6;

    println!("How the enum looks like {:?}", four);  // its v4

    let home = Address {
        kind: vierte::v4,
        address: String::from("Ramolska 3"),
    };

    println!("How the enum looks like {:?}", home);  // its v4

    let outside = sehste::V4(String::from("Dupsa"));

    println!("How the enum looks like {:?}", outside);  // its v4

    let m = Message::Write(String::from("hello"));
    m.call();

    /*
        Option<T>
    
        is an enum,
        used by the type system to indicate, all the possible
        cases

        enum Option<T> {
            Some(T),
            None,
        }
        
        it is in prelude, so you dont import that

        you can use Some and None without the Option::

        rust doesnt have Null (but has None :D )


    */

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // you have convert Option<T> to T, before working on T
    
    /*
        match

        matches values in order

        Matches are exchaustive, must have None



    */

    println!("The value is {}", value_in_cents(Coin::Penny));
    println!("The value is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:#?}, {:#?}, {:#?}, ", five, six, none);


    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),    // a placeholder sending all not matched to ()
    }

    let some_u8_value = Some(3); 
    
    // i dont want to know what uglienes created that turd
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // that turd above is this:

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // now this is fucking better:

    let mut count = 0;
    // this thing
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // can be this...
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // you can't make an if?
    
}