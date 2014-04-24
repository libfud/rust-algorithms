use common::utils::string_getter;

pub mod common {
    pub mod utils;
}

fn main() {
    let mut response: ~str;
    let mut buffer: Vec<~str> = Vec::new();

    loop {
        response = string_getter("Give me input");
        if response == ~"exit" { break }
        buffer.push(response.clone());
        if response == ~"Party!" {
            let mut i = buffer.len() - 1;
            loop {
                if i == 0 { break }
                println!("{}", match buffer.pop() {
                    Some(string) => string,
                    _            => ~"Bummer"
                });
                i -= 1;
            }
        }
    }

}
