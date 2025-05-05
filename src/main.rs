use std::io::{stdin, stdout, Write};
use grass_latte::SendTypes;

fn main() {
    for i in 0..100 {
        grass_latte::send(SendTypes::Charlie(format!("{}", i)));
    }

    println!("Press enter to exit");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
