fn main() {
    println!("Hello, world!");

    let age: u16 = 24;

    let amount_paid: u32 = 2000;

    if age < 20 {
        println!("Yo fam, you are a minor");
    } else {
        println!("Yo fam, my gee");
    }

    let paid: bool = if amount_paid > 0 {true} else {false};

    println!("paid status: {paid}");

    let m_a:i32 = do_something();

    println!("the function ran and returned this: {m_a}");
}


fn do_something() -> i32 {
    let mut age: i32 = 24;

    let result = loop {
        println!("the age is {age}");
        
        while age != 0 {

            println!("we will try to decrease the age: {age}");

            age -= 1;
        }

        if age == 0 {
            println!("Age is now zero");
        }

        let reset_age: i32 = if age == 0 {24} else {1};

        break reset_age;
    };

    return result;
}