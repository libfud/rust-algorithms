/* generates an array of random numbers */

pub mod common { pub mod utils; }

fn main() {

    let args = std::os::args();
    let mut lim = 100;
    if args.len() > 1 {
        lim = match from_str::<uint>(args[1]) {
            Some(num)   => num,
            _           => 10,
        }
    }
    let array = common::utils::array_gen(lim, 10);
    for &elem in array.iter(){
        println!("{}",elem);
    }
}
