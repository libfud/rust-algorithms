mod common {
    pub mod utils;
}

fn main() {
    let mut x = common::utils::number_getter();
    let mut y = 0;
    if x < 1 {
        println!("Using 10.");
        x = 10;
    }
    let mut upper_bound = common::utils::number_getter();
    if upper_bound < 1 { upper_bound = 10; }
    let array = common::utils::array_gen(x,upper_bound);

    loop {
        println!("Input a number between 1 and {} times the first value you used.",upper_bound);
        let z = common::utils::number_getter();
        if (z > 0 && z < (10*x+1)) { y = z; break; }
    }

    println!("{}",array[x-1]);
    
    let mut i = 0;
    while i < x-1 {
        if y == array[i] { break }
        println!("{}",array[i]);
        i += 1;
    }
    
    if array[i]== y {
        println!("Array index {} holds value {} equal to the supplied value of {}", i, array[i], y);
    }
    else { println("Sorry, didn't find it."); }
}
