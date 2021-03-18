#[derive(Debug)]
struct User {
    username: String,  //String is owned by the struct a &str would be problematic
    email: String,
    sign_in_count: u64,
    active: bool,
}

// i can use tuples like that as structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

// this is how we implement functions related to structures
impl Rectangle {
    fn area(&self) -> u32 { //
        self.width * self.height  //one liners might be cool without ;
    }

    fn stats(&self) {   // thats just reading
        println! {
            "Area: {:?}", self.area()
        };
    }

    fn change(&mut self, width: u32, height: u32) {  // this is mutable reference
        self.width = width;
        self.height = height;
    }

    fn transform(self) -> Rectangle {  // this takes ownership
        Rectangle{                     // rectangle is then useless
            ..self
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("redcat7@gmail.com");
    
    let user2 = build_user(String::from("redcat7@gmail.com"), String::from("kot"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2     // i can use rest of the fields from the struct
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Hello, world!");
    println!("This is struct {:?}", user1);
    println!("This is another struct {:?}", user2);
    println!("This is another struct {:?}", user3);

    let rectangle = Rectangle {  // nonmutable rectangle
        width: 30,
        height: 50,
    };
 
    rectangle.stats();  // display stats

    // rectangle was moved  to transformer 
    let mut transformer = rectangle.transform();

    transformer.width = 200;
    transformer.stats();

    let mut rectangle = Rectangle {  // new rectangle, mutable
        width: 30,
        height: 50,
    };

    rectangle.change(102, 34); // we can change
    rectangle.stats();
    rectangle.jupikajejmadafaka();

    //this is how i run associated function
    Rectangle::square(20);
}




fn build_user(email: String, username: String) -> User {
    User {
        email,  // i can use a filed init shorthand when variable and field have the same name
        username,
        active: false,
        sign_in_count: 10,
    }
}

/// hihiihihihi i can i do that in many different files?
/// then import them and modify original stuff??
impl Rectangle {
    fn jupikajejmadafaka(&self) {
        println!("Stirb langsam");
    }
}