// declaring that we are using clap
extern crate clap;

// shortens the use of clap turning it into a function
use clap::{App, Arg};

fn main() {
    let matches = App::new("devoid")
        .version("0.1.0")
        .author("Sam Clark <samrocksc@gmail.com>")
        .about("wget clone written in Rust")
        .arg(
            Arg::with_name("Command")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"),
        )
        .get_matches();
    let url = matches.value_of("Command").unwrap();
    println!("{}", url);

    // This function only gets compiled if the target OS is linux
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }

    // And this function only gets compiled if the target OS is *not* linux
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }

    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    devoidcli::checks::for_os(String::from(url));
}
