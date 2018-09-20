#[macro_use] extern crate clap;
use clap::App;

use core;

fn main() {
    let core = core::hello();
    println!("Hello, world!: {}", core);

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(peers) = matches.value_of("peers") {
        match peers {
            _ => println!("Peers: {:?}", peers),
        }
    } else {
        eprintln!("--peers <ports> not being used");
    }
}
