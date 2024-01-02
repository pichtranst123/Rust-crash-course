pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Peter", "VN");

    // Positional Arguments
    println!("{0} is from {1} and {0} like to {2}", "Peter", "Vn", "code");

    // Named Arguments
    println!(
        "{name} like to play {activity}",
        name = "John",
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
