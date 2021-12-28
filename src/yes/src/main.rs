use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..2);

    let mut msg = String::from("yes");
    if !args.is_empty() {
        msg = args.join(" ");
    }

    loop {
        println!("{}", msg);
    }
}
