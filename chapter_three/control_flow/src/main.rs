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
}
