mod broker;
mod manager;
mod named_pipes;
mod pc_queue;
mod protocols;
mod publisher;
mod subscriber;

fn print_usage(args: Vec<String>) {
    println!("Usage: {} [broker|manager|publisher|subscriber]", args[0]);
}

// This main only calls the run() function of the appropriate module
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage(args);
        return;
    }

    match args[1].as_str() {
        "broker" => broker::run(args),
        "manager" => manager::run(args),
        "publisher" => publisher::run(args),
        "subscriber" => subscriber::run(args),
        _ => print_usage(args),
    }
}
