mod broker;
mod manager;
mod publisher;
mod subscriber;

fn print_usage(args: Vec<String>) {
    println!("Usage: {} [broker|manager|publisher|subscriber]", args[0]);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage(args);
        return;
    }

    match args[1].as_str() {
        "broker" => broker::run(),
        "manager" => manager::run(),
        "publisher" => publisher::run(),
        "subscriber" => subscriber::run(),
        _ => print_usage(args),
    }
}
