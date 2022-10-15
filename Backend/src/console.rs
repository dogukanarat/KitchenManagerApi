use termion::{color, style};

// this function print info message with timestamp
pub async fn info(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] INFO {}{}",
        color::Fg(color::Blue),
        now.format("%Y-%m-%d %H:%M:%S"),
        style::Reset,
        msg
    );
}

// this function print error message with timestamp
pub async fn error(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] ERROR {}{}",
        color::Fg(color::Red),
        now.format("%Y-%m-%d %H:%M:%S"),
        style::Reset,
        msg
    );
}

// this function print warning message with timestamp
pub async fn warn(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] WARNING {}{}",
        color::Fg(color::Yellow),
        now.format("%Y-%m-%d %H:%M:%S"),
        style::Reset,
        msg
    );
}

// this function print success message with timestamp
pub async fn success(msg: &str) {
    let now = chrono::Local::now();
    println!(
        "{}[{}] SUCCESS {}{}",
        color::Fg(color::Green),
        now.format("%Y-%m-%d %H:%M:%S"),
        style::Reset,
        msg
    );
}