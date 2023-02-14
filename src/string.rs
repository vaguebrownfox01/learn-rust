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

    // let st = "string"; // type: &str
    let mut hello = String::from("Hell0 String!"); // string type
    println!("len: {}, {}", hello.len(), hello);
    hello.push('-'); // push char
    hello.push_str("Rust!"); // push string

    println!("len: {}, {}", hello.len(), hello);

    println!("capacity: {}", hello.capacity()); // also is_empty()

    println!("contain: {}", hello.contains("Hell0")); // also replace("this", "that")

    // loop through string
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut cs = String::with_capacity(10);
    cs.push('a');
    cs.push('b');
    println!("{}", cs);

    assert_eq!(2, cs.len());
    // assert_eq!(3, cs.len()); // thread 'main' panicked at 'assertion failed: `(left == right)`
}
