// Example for Drop trait
trait MyTrait {
    fn do_something(&self);
}

struct MyType {
    name: String,
    value: i32,
}

impl MyTrait for MyType {
    fn do_something(&self) {
        println!(
            "Doing something with name {} and value {}",
            self.name, self.value
        );
    }
}

impl Drop for MyType {
    fn drop(&mut self) {
        self.name.clear();
        self.value = 0;
        println!("Dropping MyType with cleared name and value");
    }
}

fn main() {
    let a = MyType {
        name: String::from("Alice"),
        value: 42,
    };
    a.do_something();
    // Drop run after main function
}
