// A message box stores messages that can be read by the subscriber and written by the publisher

#[derive(Debug)]
pub struct MessageBox {
    title: String,
    message: String,
}

impl MessageBox {
    pub fn new(title: String, message: String) -> Self {
        Self { title, message }
    }

    pub fn print(&self) {
        println!("{}: {}", self.title, self.message);
    }
}
