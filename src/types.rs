/*
Primitive Types--
int: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (bits in memory)
float: f32, f64
bool: (bool)
char: (char)
tuples
arrays
 */

// Rust is statically typed, all types of variables must be known at compile time; type is infered by value type
pub fn run() {
    // default i32
    let x = 1;

    // default f64
    let y = 2.5;

    // add explicit type
    let z: i64 = 454545454545454;

    println!("{}, {}, {}", x, y, z);

    // find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max f64: {}", std::f64::MAX);

    // bool
    let is_active: bool = true;
    println!("{:?}", (x, y, z, is_active, 2 > 1));

    // char
    let a = 'a';
    // let b = 'bb'; // nope; only one character;
    let face = '\u{1F600}';

    println!("{:?}", (a, face));
}
