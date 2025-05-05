use std::io::{stdin, stdout, Write};
use grass_latte::SendTypes;

fn main() {
    grass_latte::serve_webpage();
    
    stdin().read_line(&mut String::new()).ok();
    grass_latte::send(SendTypes::Alpha);

    stdin().read_line(&mut String::new()).ok();
    grass_latte::send(SendTypes::Bravo);

    stdin().read_line(&mut String::new()).ok();
    grass_latte::send(SendTypes::Charlie("text".to_string()));

    println!("Press enter to exit");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
