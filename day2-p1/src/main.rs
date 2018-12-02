use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut all_twos = 0;
    let mut all_threes = 0;

    for line in contents.lines() {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for letter in alphabet.chars() {
            counts.insert(letter, line.matches(letter).count());
        }

        //let mut twos = 0;
        //let mut threes = 0;

        let twos = counts.iter().filter(|&(_, &v)| v == 2).count();
        let threes = counts.iter().filter(|&(_, &v)| v == 3).count();
        if twos > 0 {
            all_twos += 1;
        }
        if threes > 0 {
            all_threes += 1;
        }
        println!("{}: {} {}", line, twos, threes);
    }

    println!("{}x{}={}", all_twos, all_threes, all_twos * all_threes);

    Ok(())
}
