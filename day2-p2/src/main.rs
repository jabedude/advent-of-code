use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let cpy = contents.clone();

    for id in contents.lines() {
        for line in cpy.lines() {
            let mut out = String::new();
            let mut diff = 0;
	    for (x, y) in id.chars().zip(line.chars()) {
		if x != y {
		    diff += 1i32;
		} else {
                    out.push(x);
                }
            }

            if diff == 1 {
                println!("{}, {}", id, line);
                println!("{}", out);
            }
        }
    }

    Ok(())
}
