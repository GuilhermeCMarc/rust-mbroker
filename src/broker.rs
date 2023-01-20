// This module creates a thread for each session and handles the communication
use crate::named_pipes;
use crate::pc_queue;

fn print_usage() {
    println!("Usage: broker <name> <max_sessions>");
}

pub fn run(args: Vec<String>) {
    if args.len() < 4 {
        print_usage();
        return;
    }

    let name = args[2].clone();

    let max_sessions = args[3].parse::<usize>().expect("Invalid max sessions");

    let mut queue = pc_queue::ProducerConsumerQueue::new(max_sessions);

    named_pipes::create_named_pipe(name.clone()).unwrap();

    // todo: add multithread
    // for _ in 0..max_sessions {
    //     std::thread::spawn(|| session(queue));
    // }

    loop {
        let content = named_pipes::read_from_pipe(name.clone()).unwrap();
        queue.enqueue(content).unwrap();
        queue.dequeue().unwrap();
    }
}
