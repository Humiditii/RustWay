use std::io;

fn main() {
    // variables
    let mut x = 5;
    println!(" The value of x here is: {x}");
    x = 6;
    println!("this is the new value : {x}");

    // constants
    const SERVER_PORT: i32 = 3000;

    println!("server port is : {SERVER_PORT}");

    // shadowing

    let age = 23;

    let age = age +1;

    println!("age is {age}"); 

    {
        let age = age + 10;
        println!("age is {age} in the inner scope");
    }

    // data types: scalar or vector
    // scalar could be: integer, floats, boolean, character

    let price: u16 = 257;
    println!("the price is: {price}");

    // floats: there are two types . the f32 and f64. f64 
    let a: f64 = 3.3;
    let b: f64 = 44.3;
    let c:f32 = 5.2;

    println!("float 32 is:{a} while float 64 is : {b}");

    // arithmetic operations
    let sum: f64 = a + b;

    let subtraction: f64 = b - a;

    let multiplication: f64 = a * b;

    let division: f64 = b/a;

    let modulus: u8 = 47 % 5;

    println!("Addition: {sum}");
    println!("subtraction: {subtraction}");
    println!("multiplication: {multiplication}");
    println!("division: {division}");
    println!("modulus: {modulus}");

    // boolean: bool, could be true or false
    let is_true = true;
    let is_false:bool = false;

    println!("is true value is : {is_true}");
    println!("is false value is: {is_false}");

    // character types
    let name="hameed";
    let emoji = '😂';

    println!("How are you {name} and {emoji}");

    // tuples: tuple are static in size, they can not increase in size or redice
    let tup: (u8, f64, f32) = (20,23.34, 0.04);

    let (x,y,z) = tup;

    println!("print tuple: {x},{y},{z}");

    let first_element = tup.0;

    // accessing tuples via period and index .idx
    println!("This a tuple value gotten via dot notation: `{first_element}`");

    // arrays
    let ages = [20,23,32,20];

    let agex_x:[i16; 4] = [20,30,40, 11];

    let amounts:[i32; 10] = [2000; 10];

    println!(" {amt} ", amt = amounts[1]);

    println!("enter any index within 0-4");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("failed to read line");

    let index:usize = index.trim().parse().expect("Index not a number");

    println!("element selected is {el}, with index {idx}", el=ages[index], idx=index);

}
