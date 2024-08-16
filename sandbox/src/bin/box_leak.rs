fn main() {
    let b = Box::new(42);
    let c = b.clone();

    println!("{:p} {:p}", &b, &c);
    println!("{:p} {:p}", &*b, &*c);
    let leaked: &'static mut i32 = Box::leak(b);

    *leaked = 10;

    let move_var = *leaked;

    println!("{move_var}");
}
