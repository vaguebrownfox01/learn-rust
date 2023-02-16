//Array: Fixed list where elements have same data type

use std::mem;

pub fn run() {
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];
    // let nums: [i32; 5] = [1, 2, 3]; // Gives error

    nums[2] = 24; // can change value, but not length

    println!("{:?}", nums);

    // Single value
    println!("single value {}", nums[0]);

    // Array length
    println!("array len {}", nums.len());

    // Arrays are stack allocated
    // println!("Array occupies {} bytes", std::mem::size_of_val(&nums));
    println!("Array occupies {} bytes", mem::size_of_val(&nums));

    // Slices
    let slice: &[i32] = &nums;
    println!("Slice: {:?}", slice);

    let slic: &[i32] = &nums[1..3];
    println!("Slic: {:?}", slic);
}
