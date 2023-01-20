#[derive(Debug)]
pub struct ProducerConsumerQueue {
    queue: Vec<Vec<u8>>,
    capacity: usize,
    current_size: usize,
    tail: usize,
    head: usize,
}

impl ProducerConsumerQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Vec::with_capacity(capacity),
            capacity,
            current_size: 0,
            tail: 0,
            head: 0,
        }
    }

    pub fn enqueue(&mut self, item: Vec<u8>) -> Result<(), String> {
        // Todo: implement semaphore to wait until is not full
        if self.current_size == self.capacity {
            return Err("Queue is full".to_string());
        }

        self.queue.insert(self.tail, item);
        self.tail = (self.tail + 1) % self.capacity;

        return Ok(());
    }

    pub fn dequeue(&mut self) -> Result<Vec<u8>, String> {
        // Todo: implement semaphore to wait until is not empty
        if self.current_size == 0 {
            return Err("Queue is empty".to_string());
        }

        let item = self.queue.remove(self.head);
        self.head = (self.head + 1) % self.capacity;

        return Ok(item);
    }
}
