use std::process;

use crate::protocols;
use crate::protocols::publisher_message;
use crate::protocols::register_publisher;
use crate::protocols::send_protocol;

fn print_usage() {
    println!("Usage: publisher <register-pipe> <client-pipe> <message-box>");
}

pub fn run(args: Vec<String>) {
    ctrlc::set_handler(move || {
        println!("Publisher stopped");
        process::exit(0);
    })
    .unwrap();

    if args.len() < 4 {
        print_usage();
        return;
    }

    let register_pipe = args[2].clone();
    let message_box = args[4].clone();
    let client_pipe = args[3].clone();

    send_protocol(
        register_publisher(client_pipe.clone(), message_box.clone()),
        register_pipe.clone(),
    )
    .expect("Failed to send register publisher message");

    println!("Publisher started");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let message = input.trim().to_string();

        match protocols::send_protocol(publisher_message(message.clone()), client_pipe.clone()) {
            Ok(_) => {
                println!("Publisher sent: {}", message);
            }
            Err(_) => continue,
        }
    }
}
