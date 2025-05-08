use grass_latte::{serve_webpage, serve_webpage_at_port, set_port_range};
use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;
use clap::Parser;
use derive_getters::Getters;

#[derive(Parser, Debug, Getters)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, action, help = "Port range for the server to listen on e.g. 3030-3040")]
    port_range: Option<String>,
    #[arg(short, long, action, help = "Port to serve web page on e.g. 8000")]
    web_port: Option<String>
}

fn parse_range(input: &str) -> Result<(u16, u16), String> {
    if let Some((start_str, end_str)) = input.split_once('-') {
        let start = start_str.trim().parse::<u16>().map_err(|e| e.to_string())?;
        let end = end_str.trim().parse::<u16>().map_err(|e| e.to_string())?;
        if end < start {
            return Err("End of port range cannot be less than start".to_string());
        }
        Ok((start, end))
    } else {
        let num = input.trim().parse::<u16>().map_err(|e| e.to_string())?;
        Ok((num, num))
    }
}

fn main() {
    let args = Args::parse();

    if let Some(port_range) = args.port_range() {
        let range = match parse_range(port_range) {
            Ok(r) => r,
            Err(e) => {
                println!("{e}");
                return;
            }
        };
        println!("Server will listen on ports: {}-{}", range.0, range.1);
        set_port_range(range);
    }
    else {
        println!("Server will listen on default ports");
    }

    if let Some(web_port) = args.web_port() {
        let web_port = match web_port.trim().parse::<u16>() {
            Ok(p) => p,
            Err(e) => {
                println!("{e}");
                return;
            }
        };
        
        serve_webpage_at_port(web_port);
    }
    else {
        serve_webpage();
    }

    println!("Press enter to exit server");
    stdout().flush().ok();
    stdin().read_line(&mut String::new()).ok();
}
