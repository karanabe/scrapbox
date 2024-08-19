fn main() {
    let numbers = vec![1, 2, 3];

    {
        let display_numbers = || {
            for n in numbers {
                println!("{n}");
            }
        };
        display_numbers();
    }

    println!("{:?}", numbers);
}
