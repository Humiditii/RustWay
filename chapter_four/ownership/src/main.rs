fn main() {
    let mut s = String::from("Hello"); // this uses the heap and is mutable, but string literals isn't 

    println!("{}",s);

    s.push_str("Hameed");

    println!("{}",s);

    // understanding double free error
    let name = String::from("Hameed");

    println!("{}",name);

    let ham = name.clone(); // copying the stack and heap

    println!("{}", ham);

    let len = calculate_length(&s);

    println!("{}",len);

    println!("{}", s);

    change(&mut s);

    println!("{}", s);


}


fn calculate_length(s : &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str("morning")
}

// fn call_s(){
//     s
// }