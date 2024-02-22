struct User {
        first_name: String,
        age: u16,
        last_name: String,
        email: String,
        is_active: bool
    }

struct Rectangle {
    weifth:i32,
    length: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.weifth * self.length;
    }
}

struct Colour (i32, i32, i32);
struct Point (f32, f32, f32);
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

    let _black: Colour = Colour(0,0,0);
    let _exfil_location: Point = Point(0.0, 10.4, 29.4);

    let rectangle = Rectangle {
        weifth: 20,
        length: 41
    };

    println!("{}",rectangle.area());

    #[derive(Debug)]
    struct Hameed {
        married: bool,
        age: i32,
        fiance_name: String
    }

    let mut hameed_instance = Hameed {
        married: false,
        age: 24,
        fiance_name: "sofiyyah".to_string()
    };

    println!("hameed prop is: {:?}",hameed_instance )



}


fn user_builder(fullname: String, email:String, age:u16) -> User {

    let fullname_as_bytes: &[u8] = fullname.as_bytes(); 

    let mut first_name: String = String::from("");
    let mut last_name: String =  String::from("");

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
        age, 
        is_active: true
    };
}

struct  Dimension {
    length:i32,
    breadth:i32,
    unit:String,
}

fn rectangle_dimension(area:i32, perimeter:i32) -> Dimension {

    let breadth = get_the_breadth(area, perimeter);

    Dimension {
        breadth: breadth,
        length: get_length(area, breadth),
        unit: "cm".to_string()

    }
}


fn get_the_breadth(area:i32, perimeter:i32) -> i32 {
    area / ( (area*perimeter) -1 )
}

fn get_length(area:i32, breadth:i32) -> i32 {
     if area < 1 {
        return 1;
    }else{
        return area/breadth
    }
}
