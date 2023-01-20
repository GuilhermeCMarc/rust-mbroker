use nix::sys::stat;
use nix::unistd;
use tempfile::tempdir;

pub fn create_named_pipe(name: String) -> Result<(), String> {
    let tmp_dir = tempdir().unwrap();
    let fifo_path = tmp_dir.path().join(name);

    match unistd::mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Failed to create named pipe".to_string()),
    }
}

pub fn read_from_pipe(name: String) -> Result<Vec<u8>, String> {
    let tmp_dir = tempdir().unwrap();
    let fifo_path = tmp_dir.path().join(name);

    match std::fs::read(fifo_path) {
        Ok(content) => return Ok(content),
        Err(_) => return Err("Failed to read from named pipe".to_string()),
    }
}

pub fn write_in_pipe(name: String, content: Vec<u8>) -> Result<(), String> {
    let tmp_dir = tempdir().unwrap();
    let fifo_path = tmp_dir.path().join(name);

    match std::fs::write(fifo_path, content) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Failed to write in named pipe".to_string()),
    }
}
