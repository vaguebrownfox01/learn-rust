pub fn format() {
    // Basic
    println!("hello from format: Insert number here {}", 1);

    println!("{} is {}", "rust", "cool");

    // Positional arguments
    println!("{0} is not {0} anymore :(, says {1}", "jeevan", "env");

    // Named arguments
    println!("{name} | a {ego}", name = "env", ego = "vrx");

    // Placeholder traits
    println!("Binary: {:b}; Hex: {:x}; Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
