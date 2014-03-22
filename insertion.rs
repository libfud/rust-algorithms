pub mod common { pub mod utils; }

fn main() {

    let x = 100000;
    let pathname = ~"common/numbers.txt";
    let mut array = common::utils::int_array_from_file(pathname,x);
    for &elem in array.iter(){
        println!("{}",elem);
    }

    let mut i = 0;
    while i < x {
        let val = array[i];
        let mut j = i - 1;
        while (j >= 0 && array[j] > val) {
            array[j+1] = array[j];
            j -= 1;
        }
        array[j+1] = val;
        i+=1;
    }

    for &elem in array.iter(){
        println!("{}",elem);
    }
}
