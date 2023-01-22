use std::fs::create_dir;
use std::fs::remove_dir_all;
use std::process;

// This module creates a thread for each session and handles the communication
use crate::named_pipes;
use crate::pc_queue;

fn print_usage() {
    println!("Usage: broker <name> <max_sessions>");
}

pub fn run(args: Vec<String>) {
    ctrlc::set_handler(move || {
        println!("Broker stopped");
        remove_dir_all("./tmp").expect("Error removing /tmp directory");
        process::exit(0);
    })
    .unwrap();

    if args.len() < 4 {
        print_usage();
        return;
    }

    let name = args[2].clone();

    let max_sessions = args[3].parse::<usize>().expect("Invalid max sessions");

    let mut queue = pc_queue::ProducerConsumerQueue::new(max_sessions);

    // Creates the tmp dir (where the named pipes will be created)
    create_dir("./tmp").expect("Error creating /tmp directory");

    named_pipes::create_named_pipe(name.clone()).unwrap();

    // todo: add multithread
    // for _ in 0..max_sessions {
    //     std::thread::spawn(|| session(queue));
    // }

    println!("Broker started");
    loop {
        let content: Vec<u8>;
        match named_pipes::read_from_pipe(name.clone()) {
            Ok(c) => {
                println!("Broker read: {:?}", c);
                content = c;
                queue.enqueue(content).unwrap();
            }
            Err(_) => continue,
        }
    }
}
