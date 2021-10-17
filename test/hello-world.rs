//import module
mod hello;

fn main() {
    println!("Hello, world!");
    hello::say_hello(); // call function in module

    let num1: i32 = 3;
    let num1: f32 = 3.2;
    let num1 = 4;

    let _u: Vec<i32> = Vec::new();

    let _a: i32 = 5;
    
    let _b: [i32; 3] = [1, 2, 3];

    let _c: [&str; 2] = ["engineer", "man"]; // use tulips () to figure out type.

    //string stuff    
    let name1: &'static str = "engineer man"; //string in binary, just a slice

    let mut name2: String = String::new(); //string in heap
    name2 = name2 + "engineer";
    name2.push_str(" man");
    println!("{}", name2);

    let name3: &str = &name1[..8];
    let name4: &str = &name1[9..];
    println!("{}", name3);
    println!("{}", name4);

    println!("{}", add(1, 2));
    println!("{}", add(3, 4));

    let mut num = 1;
    inc(&mut num);

    enum Color {
        Red,
        Green,
        Blue,
        Orange,
        /*Custom(String), // tuple struct style
        Coord{x: i32, y: i32} // classic struct style*/
    }

    println!("{}", Color::Green as i32);

    enum Number {
        One = 1,
        Five = 5,
        Ten = 0xA,
    }

    let favorite: Color = Color::Green;
    let custom: Color = Color::Custom("pink".to_string());

    if let Color::Green = favorite { // works only 1 way
        println!("favorite color is green");
    }

    match favorite {
        Color::Green => { // arrow function
            println!("favorite color is green");
            println!("favorite color is green");
        },
        Color::Blue => println!("favorite color is blue"),
        _ => {} // default
    }

    //match very powerful
    match custom {
        Color::Custom(color) => println!("custom color: {}", color), //match using arguments
        _ => {} // default
    }

    //built-in Option<T> enum type (for null)
    let mut age: Option<i32> = None;
    // do proccesing
    age = Some(18);

    match age {
        Some(age) => {
            if age >= 18 {
                println!("Can have beer");
            } else {
                println!("Can't have beer, is only {}", age);
            }
        },
        None => println!("unknown age")
    }
}

//functions (& means borrowing)
fn get_length(s: &str) -> usize {
    return s.chars().count();
}

fn take_ownership(s: String) { // deallocates memory
    println!("{}", s);
}

fn borrow_name(s: &String) { // doesn't deallocate name
    println!("{}", s);
}

fn inc(n1: &mut i32) {
    *n1 = *n1+ 1
}

fn add(n1: i32, n2:i32) -> i32 {
    n1+n2
}