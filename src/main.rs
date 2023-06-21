#![deny(clippy::all)]

const TEST_CONST_VALUE: &str = "It's a const value";

fn main() {
    let name = "Jonatas";

    print_name(name);

    println!("Hey it's {name} here!");

    println!("{}", TEST_CONST_VALUE);

    let my_tupples = ("It's a tupple value", 10);

    println!("{}, {}", my_tupples.0, my_tupples.1);

    /*
    This will not compile because we're traing to acces a value that has moved to another location in memory
    let test_string = String::from("Test borrow");
    let test_borrow = test_string;
    println!("Borrow: {}", test_string)
    */

    let test_string = "Test borrow".to_string();
    let test_borrow = test_string;
    println!("Borrow: {}", test_borrow);

    {
        let block_value = "Testimg block";
        println!("{name}");
        println!("{block_value}");
    }

    /*
    This will fail, block_value it out of scope at this line.
    println!("{block_value}");
    */

    let user = User {
        name: "Jonatas".to_string(),
        age: 30,
    };

    print!("A struct: {}, {}", user.name, user.age);
}

fn print_name(name: &str) {
    println!("It's my name: {name}");
}

struct User {
    name: String,
    age: i32,
}
