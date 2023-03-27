use std::{ io::{ stdout, Write }, thread::sleep, time::Duration };

fn main() {
    let mut stdout = stdout();
    let mut bar = String::new();
    let mut bar2 = String::new();
    for i in (0..=100).into_iter() {
        bar = "█".repeat(i / 2);
        bar2 = " ".repeat(50 - i / 2);
        print!("\r\x1b[93m[{}{}] {i}%... \x1b[0m", &bar, &bar2);
        stdout.flush().unwrap();
        sleep(Duration::from_millis(20));
    }
    println!("\r\x1b[32m[{}{}] 100%       \x1b[0m", &bar, &bar2);
}