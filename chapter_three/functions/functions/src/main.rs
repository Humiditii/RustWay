fn main() {
    println!("Hello, world!");

    another_function();

    number_printer(20);

    print_measurement_unit(200, 'k');

    println!("room temperature is: {tmp}:{unit}", tmp = room_temperature(), unit = "c");

    let room_temp: i32 =  temperature_convertor(25);

    println!("room temperature is: {room_temp}");
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

fn temperature_convertor(temp:i32) -> i32 {
    return temp + 273
}

// statement must have a semicolon