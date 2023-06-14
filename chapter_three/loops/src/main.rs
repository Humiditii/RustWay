fn main() {
    let mut counter: u16 = 0;
    let result = loop {
        if counter == 10 {
            break counter * 2;
        }
        counter += 1;
    };

    println!("result is: {result}");

    rocket_liftoff_countdown(10);

    get_each_element();

    call_all_names();

    range_loop();
}


fn rocket_liftoff_countdown(mut time: u32){
    while time != 0 {
        println!("{time}s to liftoff...");
        time -= 1;
    }
    println!("LIFTOFF!!!")
}

fn get_each_element(){
    let arr:[u16;10] = [0;10];
    let mut index = 0;
    while index < 10 {
        println!("getting element index: {index}, value: {el}", el = arr[index]);

        index +=1;

    }

}

fn call_all_names(){

    let names:[&str;5] = ["hameed", "ola", "musa", "lola", "jide"];

    for name in names {
        println!("Hello my fam {name}");
    }
}


fn range_loop(){
    for i in 1..5 {
        println!("{i}")
    }
}