use std::thread;

static Y: i32 = 11;

fn main() {
    static X: i32 = 12;

    thread::spawn(move || dbg!(&Y));
    thread::spawn(move || dbg!(&X));

    println!("{X}");
}
