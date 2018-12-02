use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let nums: Vec<i32> = contents.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut to_find = HashSet::new();
    to_find.insert(0);
    let mut sum: i32 = 0;

    'outer: loop {
        'inner: for num in &nums {
            sum += num;
            if to_find.contains(&sum) {
                break 'outer;
            }
            to_find.insert(sum);
        }

    }

    println!("{:?}", sum);
    Ok(())
}
