pub fn run() {
    let ego: (&str, &str, i8) = ("Env", "vrx", 25);

    println!("{} is a {}, created at {}", ego.0, ego.1, ego.2);
}