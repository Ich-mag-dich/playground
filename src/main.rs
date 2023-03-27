use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();
    let mut bar = String::new();
    let mut bar2 = String::new();
    for i in (0..=100).into_iter() {
        bar = "█".repeat(i / 2);
        bar2 = " ".repeat(50 - i / 2);
        print!("\r[{}{}] {i}%...", &bar, &bar2);
        stdout.flush().unwrap();
        sleep(Duration::from_millis(20));
    }
    println!("\r[{}{}] 100%", &bar, &bar2);
}
