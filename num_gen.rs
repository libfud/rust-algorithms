pub mod common { pub mod utils; }

fn main() {
    let array = common::utils::array_gen(100000, 10);
    for &elem in array.iter(){
        println!("{}",elem);
    }
}
