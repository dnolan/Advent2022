use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    let mut biggest: i32 = 0;

    if let Ok(lines) = read_lines("data.txt") {
        for line in lines {
            if let Ok(calories) = line {

                if calories == "" {
                    elves.push(total);
                    total = 0;
                    
                    continue;
                }

                total += calories.parse::<i32>().unwrap();

                if total > biggest {
                    biggest = total;
                    println!("Bigger:{}", biggest)                    ;
                }
            }
        }
    }

    println!("Biggest {}", biggest);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
