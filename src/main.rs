use grass_latte::{serve_webpage, set_port_range};
use std::io::{stdin, stdout, Write};

fn main() {
    set_port_range((3030, 3030));
    serve_webpage();

    grass_latte::send_node(["Alpha", "Bravo", "Charlie"]);
    grass_latte::send_text(["Alpha", "Bravo", "Xray"], "xray: 5");
    grass_latte::send_node(["Alpha", "Bravo", "Delta"]);
    grass_latte::send_text(["Alpha", "Bravo", "Hotel"], "hotel: ");
    grass_latte::send_text(["Alpha", "Bravo", "Hotel", "Whiskey"], "whiskey");

    println!("Press enter to exit");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
