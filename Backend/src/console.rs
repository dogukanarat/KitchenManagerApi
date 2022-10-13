use termion::{color, style};

// this function print info message with timestamp
pub async fn info(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] {}{}",
        color::Fg(color::Green),
        now.format("%Y-%m-%d %H:%M:%S"),
        msg,
        style::Reset
    );
}

// this function print error message with timestamp
pub async fn error(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] {}{}",
        color::Fg(color::Red),
        now.format("%Y-%m-%d %H:%M:%S"),
        msg,
        style::Reset
    );
}

// this function print warning message with timestamp
pub async fn warn(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] {}{}",
        color::Fg(color::Yellow),
        now.format("%Y-%m-%d %H:%M:%S"),
        msg,
        style::Reset
    );
}