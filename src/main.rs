use std::io::{stdin, stdout, Write};
use grass_latte::{serve_webpage, SendTypes};

fn main() {
    serve_webpage();
    
    for i in 0..100 {
        grass_latte::send(SendTypes::Charlie(format!("{}", i)));
    }

    println!("Press enter to exit");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
