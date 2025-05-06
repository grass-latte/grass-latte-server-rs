use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use grass_latte::{serve_webpage, set_port_range};

fn main() {
    set_port_range((3030, 3030));
    serve_webpage();

    grass_latte::send_node(["a", "b", "c"]);
    grass_latte::send_node(["a", "d"]);

    thread::sleep(Duration::from_millis(5000));
    grass_latte::delete_element(["a", "b", "c"]);

    println!("Press enter to exit");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
