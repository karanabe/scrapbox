// Import the generated bindings
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    // Calls C++ functions
    unsafe {
        bindings::foo();
    }
}
