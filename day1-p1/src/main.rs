use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let nums: Vec<i32> = contents.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    let sum: i32 = nums.iter().sum();
    println!("{:?}", sum);
    Ok(())
}
