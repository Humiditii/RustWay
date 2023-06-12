struct User {
        first_name: String,
        age: u16,
        last_name: String,
        email: String,
        is_active: bool
    }
fn main() {
    println!("Hello, world!");

    let first_user = User {
        first_name: String::from("Hameed"),
        last_name: String::from("Babatunde"),
        age: 23,
        is_active: true,
        email:String::from("humiditii45@gmail.com")
    };

    first_user.email;

    let hameed_user: User = user_builder( String::from("Hameed Babatunde"),String::from("humiditii45@gmail.com"), 23);
    let e = hameed_user.first_name;
    println!("{}",e);
}


fn user_builder(fullname: String, email:String, age:u16) -> User {

    let fullname_as_bytes: &[u8] = fullname.as_bytes(); 

    let mut first_name = String::from("");
    let mut last_name =  String::from("");

    first_name.push_str(&fullname);
    last_name.push_str(&fullname);

    for (i, &item) in fullname_as_bytes.iter().enumerate() {
        if item == b' ' {
            
            first_name.clear();
            last_name.clear();

            first_name.push_str(&fullname[..i]); 
            last_name.push_str(&fullname[i..fullname.len()]);

            break;
        }
    };

    return User {
        first_name,
        last_name,
        email: email,
        age: 23, 
        is_active: true
    };
}