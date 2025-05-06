use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use grass_latte::{serve_webpage};

fn main() {
    serve_webpage();

    grass_latte::send_node(["a", "b", "c"]);
    grass_latte::send_node(["a", "d"]);

    thread::sleep(Duration::from_millis(5000));
    grass_latte::delete_element(["a", "b", "c"]);

    println!("Press enter to exit");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
