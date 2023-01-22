use nix::sys::stat;
use nix::unistd;

use std::fs;
use std::path::PathBuf;

pub fn create_named_pipe(name: String) -> Result<(), String> {
    let tmp_dir = PathBuf::from("./tmp");
    let fifo_path = tmp_dir.join(name);

    match unistd::mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Failed to create named pipe".to_string()),
    }
}

pub fn read_from_pipe(name: String) -> Result<Vec<u8>, String> {
    let tmp_dir = PathBuf::from("/tmp");
    let fifo_path = tmp_dir.join(name);

    match std::fs::read(fifo_path) {
        Ok(content) => return Ok(content),
        Err(_) => return Err("Failed to read from named pipe".to_string()),
    }
}

pub fn write_in_pipe(name: String, content: Vec<u8>) -> Result<(), String> {
    let tmp_dir = PathBuf::from("/tmp");
    let fifo_path = tmp_dir.join(name);

    match std::fs::write(fifo_path, content) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Failed to write in named pipe".to_string()),
    }
}
