//Vector: Resizable arrays

use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let nums: [i32; 5] = [1, 2, 3]; // Gives error

    nums[2] = 24; // can change value, but not length

    println!("{:?}", nums);

    // Single value
    println!("single value {}", nums[0]);

    // vector length
    println!("vector len {}", nums.len());

    // vectors are stack allocated
    // println!("vectors occupies {} bytes", std::mem::size_of_val(&nums));
    println!("vector occupies {} bytes", mem::size_of_val(&nums));

    // Slices
    let slice: &[i32] = &nums;
    println!("Slice: {:?}", slice);

    let slic: &[i32] = &nums[1..3];
    println!("Slic: {:?}", slic);

    // push to vector
    nums.push(42);
    println!("vector {:?}", nums);
    println!("vector len {}", nums.len());

    nums.pop();
    println!("vector val pop {:?}", nums);

    // loop through vector values
    for x in nums.iter() {
        println!("num: {}", x);
    }

    // loop and mut
    for x in nums.iter_mut() {
        *x = *x * 2;
    }
    println!("vector val loop mut {:?}", nums);
}
