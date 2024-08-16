use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(5));

    let data_read = Arc::clone(&data);
    let read_thread = thread::spawn(move || {
        let read_guard = data_read.read().unwrap();
        println!("Read value: {}", *read_guard); // Deref を利用して値にアクセス
                                                 // read_guard は不変参照なので値を変更することはできません
    });

    let data_write = Arc::clone(&data);
    let write_thread = thread::spawn(move || {
        let mut write_guard = data_write.write().unwrap();
        *write_guard += 1; // DerefMut を利用して値を変更
        println!("Write value: {}", *write_guard);
        // write_guard は可変参照なので値を変更できます
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();

    // メインスレッドでも値を読み取る
    let final_value = data.read().unwrap();
    println!("Final value: {}", *final_value);
}
