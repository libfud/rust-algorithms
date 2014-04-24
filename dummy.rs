fn print_message(number: uint) {
    println!("Task number {}!", number);
    let mut i = 0;
    while i < 50000000 {
        i += 1;
    }
    println!("fuck yeah! Task number {}", number);
}

fn main() {

    let mut i = 0;
    while i < 5 { 
        spawn(proc() print_message(i));
        i += 1;
    }
    
}
