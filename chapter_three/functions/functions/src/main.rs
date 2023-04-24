fn main() {
    println!("Hello, world!");

    another_function();

    number_printer(20);

    print_measurement_unit(200, 'k');

    println!("room temperature is: {tmp}:{unit}", tmp = room_temperature(), unit = "c");
}

fn another_function(){
    println!("This is another function");
}

fn number_printer(x: i32){
    println!("The argument passed in is: {x}")
}

fn print_measurement_unit(temp: i16, unit:char){
    println!("The temperature is {temp} and unit: {unit}")
}

fn room_temperature() -> u32 {
    25
}

// statement must have a semicolon